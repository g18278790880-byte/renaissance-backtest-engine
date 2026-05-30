use super::*;
use crate::model::Tick;
use crate::model::{OrderRequest, Side};
use crate::strategy::ThresholdStrategy;

fn assert_new_order_update(
    event: &Event,
    order_id: u64,
    remaining_quantity: u64,
    expected_message: &str,
) {
    match event {
        Event::OrderUpdate(update) => {
            assert_eq!(update.order_id, order_id);
            assert_eq!(update.status, OrderStatus::New);
            assert_eq!(update.filled_quantity, 0);
            assert_eq!(update.remaining_quantity, remaining_quantity);
            assert_eq!(update.timestamp, 0);
        }
        _ => panic!("{expected_message}"),
    }
}

#[test]
fn engine_adds_order_request_to_order_book() {
    let mut engine = Engine::new();

    let event = Event::OrderRequest(OrderRequest {
        symbol: String::from("BTCUSDT"),
        side: Side::Buy,
        price: 100_000,
        quantity: 1,
    });

    let output_events = engine.handle_event(event).unwrap();

    assert_eq!(output_events.len(), 1);
    assert_new_order_update(&output_events[0], 1, 1, "expected new order update");
    assert_eq!(engine.order_count(), 1);
    assert_eq!(engine.best_bid(), Some(100_000));
}

#[test]
fn engine_matches_crossed_orders_and_emits_trade_and_order_updates() {
    let mut engine = Engine::new();

    engine
        .handle_event(Event::OrderRequest(OrderRequest {
            symbol: String::from("BTCUSDT"),
            side: Side::Buy,
            price: 100_000,
            quantity: 2,
        }))
        .unwrap();

    let output_events = engine
        .handle_event(Event::OrderRequest(OrderRequest {
            symbol: String::from("BTCUSDT"),
            side: Side::Sell,
            price: 99_000,
            quantity: 1,
        }))
        .unwrap();

    assert_eq!(output_events.len(), 4);
    assert_new_order_update(&output_events[0], 2, 1, "expected new order update");

    match &output_events[1] {
        Event::Trade(trade) => {
            assert_eq!(trade.buy_order_id, 1);
            assert_eq!(trade.sell_order_id, 2);
            assert_eq!(trade.price, 99_000);
            assert_eq!(trade.quantity, 1);
        }
        _ => panic!("expected trade event"),
    }

    match &output_events[2] {
        Event::OrderUpdate(update) => {
            assert_eq!(update.order_id, 1);
            assert_eq!(update.status, OrderStatus::PartiallyFilled);
            assert_eq!(update.filled_quantity, 1);
            assert_eq!(update.remaining_quantity, 1);
        }
        _ => panic!("expected buy order update"),
    }

    match &output_events[3] {
        Event::OrderUpdate(update) => {
            assert_eq!(update.order_id, 2);
            assert_eq!(update.status, OrderStatus::Filled);
            assert_eq!(update.filled_quantity, 1);
            assert_eq!(update.remaining_quantity, 0);
        }
        _ => panic!("expected sell order update"),
    }

    assert_eq!(engine.order_count(), 1);
    assert_eq!(engine.best_bid(), Some(100_000));
    assert_eq!(engine.best_ask(), None);
}

#[test]
fn engine_process_market_tick_generates_order_from_strategy() {
    let mut engine = Engine::new();

    let mut strategy = ThresholdStrategy::new(String::from("BTCUSDT"), 99_000, 101_000, 1);

    let tick = Tick {
        symbol: String::from("BTCUSDT"),
        price: 98_000,
        quantity: 1,
        timestamp: 1_717_000_000,
    };

    let output_events = engine.process_market_tick(&tick, &mut strategy).unwrap();

    assert_eq!(output_events.len(), 1);
    assert_new_order_update(&output_events[0], 1, 1, "expected new order update");
    assert_eq!(engine.order_count(), 1);
    assert_eq!(engine.best_bid(), Some(98_000));
    assert_eq!(engine.best_ask(), None);
}

struct CrossStrategy {
    call_count: usize,
}

impl CrossStrategy {
    fn new() -> Self {
        Self { call_count: 0 }
    }
}

impl Strategy for CrossStrategy {
    fn on_tick(&mut self, tick: &Tick) -> Vec<OrderRequest> {
        self.call_count += 1;

        if self.call_count == 1 {
            vec![OrderRequest {
                symbol: tick.symbol.clone(),
                side: Side::Buy,
                price: 100_000,
                quantity: 2,
            }]
        } else {
            vec![OrderRequest {
                symbol: tick.symbol.clone(),
                side: Side::Sell,
                price: 99_000,
                quantity: 1,
            }]
        }
    }
}

#[test]
fn engine_process_market_tick_can_drive_strategy_and_matching() {
    let mut engine = Engine::new();
    let mut strategy = CrossStrategy::new();

    let tick1 = Tick {
        symbol: String::from("BTCUSDT"),
        price: 100_000,
        quantity: 1,
        timestamp: 1,
    };

    let tick2 = Tick {
        symbol: String::from("BTCUSDT"),
        price: 99_000,
        quantity: 1,
        timestamp: 2,
    };

    let output_events = engine.process_market_tick(&tick1, &mut strategy).unwrap();

    assert_eq!(output_events.len(), 1);
    assert_new_order_update(&output_events[0], 1, 2, "expected new order update");
    assert_eq!(engine.order_count(), 1);
    assert_eq!(engine.best_bid(), Some(100_000));

    let output_events = engine.process_market_tick(&tick2, &mut strategy).unwrap();

    assert_eq!(output_events.len(), 4);
    assert_new_order_update(&output_events[0], 2, 1, "expected new order update");

    match &output_events[1] {
        Event::Trade(trade) => {
            assert_eq!(trade.buy_order_id, 1);
            assert_eq!(trade.sell_order_id, 2);
            assert_eq!(trade.price, 99_000);
            assert_eq!(trade.quantity, 1);
        }
        _ => panic!("expected trade event"),
    }

    match &output_events[2] {
        Event::OrderUpdate(update) => {
            assert_eq!(update.order_id, 1);
            assert_eq!(update.status, OrderStatus::PartiallyFilled);
            assert_eq!(update.filled_quantity, 1);
            assert_eq!(update.remaining_quantity, 1);
            assert_eq!(update.timestamp, 0);
        }
        _ => panic!("expected buy order update"),
    }

    match &output_events[3] {
        Event::OrderUpdate(update) => {
            assert_eq!(update.order_id, 2);
            assert_eq!(update.status, OrderStatus::Filled);
            assert_eq!(update.filled_quantity, 1);
            assert_eq!(update.remaining_quantity, 0);
            assert_eq!(update.timestamp, 0);
        }
        _ => panic!("expected sell order update"),
    }

    assert_eq!(engine.order_count(), 1);
    assert_eq!(engine.best_bid(), Some(100_000));
    assert_eq!(engine.best_ask(), None);
}
