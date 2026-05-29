use crate::event::Event;
use crate::model::{OrderRequest, OrderStatus, OrderUpdate, Tick};
use crate::order_book::{OrderBook, OrderBookError};
use crate::strategy::Strategy;

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
        let mut output_events = Vec::new();

        while let Some(trade) = self
            .order_book
            .match_best_orders(self.next_trade_id, timestamp)
        {
            let buy_update = self.build_order_update(trade.buy_order_id, trade.quantity, timestamp);

            let sell_update =
                self.build_order_update(trade.sell_order_id, trade.quantity, timestamp);

            self.next_trade_id += 1;

            output_events.push(Event::Trade(trade));
            output_events.push(Event::OrderUpdate(buy_update));
            output_events.push(Event::OrderUpdate(sell_update));
        }

        Ok(output_events)
    }

    fn build_order_update(
        &self,
        order_id: u64,
        filled_quantity: u64,
        timestamp: u64,
    ) -> OrderUpdate {
        match self.order_book.get_order(order_id) {
            Some(order) => OrderUpdate {
                order_id,
                status: order.status,
                filled_quantity,
                remaining_quantity: order.quantity,
                timestamp,
            },
            None => OrderUpdate {
                order_id,
                status: OrderStatus::Filled,
                filled_quantity,
                remaining_quantity: 0,
                timestamp,
            },
        }
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

    pub fn process_market_tick<S: Strategy>(
        &mut self,
        tick: &Tick,
        strategy: &mut S,
    ) -> Result<Vec<Event>, EngineError> {
        let requests = strategy.on_tick(tick);

        let mut output_events = Vec::new();

        for request in requests {
            let events = self.handle_event(Event::OrderRequest(request))?;
            output_events.extend(events);
        }

        Ok(output_events)
    }
}

#[cfg(test)]
mod tests;
