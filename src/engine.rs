use crate::event::Event;
use crate::model::OrderRequest;
use crate::order_book::{OrderBook, OrderBookError};

#[derive(Debug, PartialEq, Eq)]
pub enum EngineError {
    OrderBook(OrderBookError),
}

#[derive(Debug)]
pub struct Engine {
    order_book: OrderBook,
    next_order_id: u64,
    next_trade_id: u64,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            order_book: OrderBook::new(),
            next_order_id: 1,
            next_trade_id: 1,
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Result<Vec<Event>, EngineError> {
        match event {
            Event::MarketTick(_tick) => Ok(Vec::new()),
            Event::OrderRequest(request) => self.handle_order_request(request),
            Event::OrderUpdate(_update) => Ok(Vec::new()),
            Event::Trade(_trade) => Ok(Vec::new()),
        }
    }

    fn handle_order_request(&mut self, request: OrderRequest) -> Result<Vec<Event>, EngineError> {
        let order_id = self.next_order_id;
        self.next_order_id += 1;

        let order = request.into_order(order_id);

        self.order_book
            .add_order(order)
            .map_err(EngineError::OrderBook)?;

        let timestamp = 0;
        let trades = self.order_book.match_orders(self.next_trade_id, timestamp);

        self.next_trade_id += trades.len() as u64;

        let events = trades.into_iter().map(Event::Trade).collect();

        Ok(events)
    }

    pub fn order_count(&self) -> usize {
        self.order_book.order_count()
    }

    pub fn best_bid(&self) -> Option<i64> {
        self.order_book.best_bid()
    }

    pub fn best_ask(&self) -> Option<i64> {
        self.order_book.best_ask()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{OrderRequest, Side};

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

        assert!(output_events.is_empty());
        assert_eq!(engine.order_count(), 1);
        assert_eq!(engine.best_bid(), Some(100_000));
    }

    #[test]
    fn engine_matches_crossed_orders_and_emits_trade_event() {
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

        assert_eq!(output_events.len(), 1);

        match &output_events[0] {
            Event::Trade(trade) => {
                assert_eq!(trade.buy_order_id, 1);
                assert_eq!(trade.sell_order_id, 2);
                assert_eq!(trade.price, 99_000);
                assert_eq!(trade.quantity, 1);
            }
            _ => panic!("expected trade event"),
        }

        assert_eq!(engine.order_count(), 1);
        assert_eq!(engine.best_bid(), Some(100_000));
        assert_eq!(engine.best_ask(), None);
    }
}
