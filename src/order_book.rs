use std::collections::BTreeMap;

use crate::model::Order;

#[derive(Debug)]
pub struct OrderBook {
    pub bids: BTreeMap<i64, Vec<Order>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.bids
            .entry(order.price)
            .or_insert(Vec::new())
            .push(order);
    }

    pub fn best_bid(&self) -> Option<i64> {
        self.bids.keys().next_back().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{OrderStatus, Side};

    #[test]
    fn best_bid_returns_highest_buy_price() {
        let mut order_book = OrderBook::new();

        let order1 = Order {
            id: 1,
            symbol: String::from("BTCUSDT"),
            side: Side::Buy,
            price: 99_000,
            quantity: 1,
            status: OrderStatus::New,
        };

        let order2 = Order {
            id: 2,
            symbol: String::from("BTCUSDT"),
            side: Side::Buy,
            price: 100_000,
            quantity: 1,
            status: OrderStatus::New,
        };

        order_book.add_order(order1);
        order_book.add_order(order2);

        assert_eq!(order_book.best_bid(), Some(100_000));
    }

    #[test]
    fn best_bid_returns_none_when_order_book_is_empty() {
        let order_book = OrderBook::new();

        assert_eq!(order_book.best_bid(), None);
    }
}
