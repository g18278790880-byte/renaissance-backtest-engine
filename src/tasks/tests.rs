use super::*;
use crate::model::OrderStatus;
use crate::strategy::DemoCrossStrategy;
use tokio::time::{timeout, Duration};

async fn recv_event(event_rx: &mut mpsc::Receiver<Event>) -> Event {
    timeout(Duration::from_secs(1), event_rx.recv())
        .await
        .expect("timed out waiting for event")
        .expect("event channel closed before receiving expected event")
}

fn assert_new_order_update(event: Event, order_id: u64, remaining_quantity: u64) {
    assert_order_update(event, order_id, OrderStatus::New, 0, remaining_quantity);
}

fn assert_trade(
    event: Event,
    trade_id: u64,
    buy_order_id: u64,
    sell_order_id: u64,
    price: i64,
    quantity: u64,
) {
    match event {
        Event::Trade(trade) => {
            assert_eq!(trade.trade_id, trade_id);
            assert_eq!(trade.buy_order_id, buy_order_id);
            assert_eq!(trade.sell_order_id, sell_order_id);
            assert_eq!(trade.symbol, "BTCUSDT");
            assert_eq!(trade.price, price);
            assert_eq!(trade.quantity, quantity);
        }
        _ => panic!("expected trade event"),
    }
}

fn assert_order_update(
    event: Event,
    order_id: u64,
    status: OrderStatus,
    filled_quantity: u64,
    remaining_quantity: u64,
) {
    match event {
        Event::OrderUpdate(update) => {
            assert_eq!(update.order_id, order_id);
            assert_eq!(update.status, status);
            assert_eq!(update.filled_quantity, filled_quantity);
            assert_eq!(update.remaining_quantity, remaining_quantity);
            assert_eq!(update.timestamp, 0);
        }
        _ => panic!("expected order update event"),
    }
}

#[tokio::test]
async fn async_pipeline_emits_order_lifecycle_events_in_order() {
    let (tick_tx, tick_rx) = mpsc::channel::<Tick>(10);
    let (order_tx, order_rx) = mpsc::channel::<OrderRequest>(10);
    let (event_tx, mut event_rx) = mpsc::channel::<Event>(10);

    let simulator = MarketDataSimulator::demo_cross_ticks();
    let market_data_handle = tokio::spawn(market_data_task(simulator, tick_tx));
    let strategy = DemoCrossStrategy::new();
    let strategy_handle = tokio::spawn(strategy_task(strategy, tick_rx, order_tx));
    let execution_handle = tokio::spawn(execution_task(order_rx, event_tx));

    let first_event = recv_event(&mut event_rx).await;
    let second_event = recv_event(&mut event_rx).await;
    let third_event = recv_event(&mut event_rx).await;
    let fourth_event = recv_event(&mut event_rx).await;
    let fifth_event = recv_event(&mut event_rx).await;

    assert_new_order_update(first_event, 1, 2);
    assert_new_order_update(second_event, 2, 1);
    assert_trade(third_event, 1, 1, 2, 99_000, 1);
    assert_order_update(fourth_event, 1, OrderStatus::PartiallyFilled, 1, 1);
    assert_order_update(fifth_event, 2, OrderStatus::Filled, 1, 0);

    market_data_handle.await.unwrap();
    strategy_handle.await.unwrap();
    execution_handle.await.unwrap();

    let no_more_events = timeout(Duration::from_secs(1), event_rx.recv())
        .await
        .expect("timed out waiting for event channel to close");

    assert!(no_more_events.is_none());
}
