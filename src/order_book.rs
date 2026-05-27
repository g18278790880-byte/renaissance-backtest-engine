use std::collections::BTreeMap;

use crate::model::{Order, Side};

#[derive(Debug)]
pub struct OrderBook {
    pub bids: BTreeMap<i64, Vec<Order>>,
    pub asks: BTreeMap<i64, Vec<Order>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        match order.side {
            Side::Buy => {
                self.bids.entry(order.price).or_default().push(order);
            }
            Side::Sell => {
                self.asks.entry(order.price).or_default().push(order);
            }
        }
    }

    pub fn best_bid(&self) -> Option<i64> {
        self.bids.keys().next_back().copied()
    }

    pub fn best_ask(&self) -> Option<i64> {
        self.asks.keys().next().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{OrderStatus, Side};

    fn create_order(id: u64, side: Side, price: i64) -> Order {
        Order {
            id,
            symbol: String::from("BTCUSDT"),
            side,
            price,
            quantity: 1,
            status: OrderStatus::New,
        }
    }

    #[test]
    fn best_bid_returns_highest_buy_price() {
        let mut order_book = OrderBook::new();

        order_book.add_order(create_order(1, Side::Buy, 99_000));
        order_book.add_order(create_order(2, Side::Buy, 100_000));

        assert_eq!(order_book.best_bid(), Some(100_000));
    }

    #[test]
    fn best_ask_returns_lowest_sell_price() {
        let mut order_book = OrderBook::new();

        order_book.add_order(create_order(1, Side::Sell, 102_000));
        order_book.add_order(create_order(2, Side::Sell, 101_000));

        assert_eq!(order_book.best_ask(), Some(101_000));
    }

    #[test]
    fn empty_order_book_has_no_best_bid_or_ask() {
        let order_book = OrderBook::new();

        assert_eq!(order_book.best_bid(), None);
        assert_eq!(order_book.best_ask(), None);
    }
}
