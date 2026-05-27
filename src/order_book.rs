use std::collections::BTreeMap;

use crate::model::{Order, Side};

#[derive(Debug, PartialEq, Eq)]
pub struct DepthLevel {
    pub price: i64,
    pub total_quantity: u64,
    pub order_count: usize,
}

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

    pub fn spread(&self) -> Option<i64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }

    pub fn bid_depth(&self, levels: usize) -> Vec<DepthLevel> {
        self.bids
            .iter()
            .rev()
            .take(levels)
            .map(|(&price, orders)| DepthLevel {
                price,
                total_quantity: orders.iter().map(|order| order.quantity).sum(),
                order_count: orders.len(),
            })
            .collect()
    }

    pub fn ask_depth(&self, levels: usize) -> Vec<DepthLevel> {
        self.asks
            .iter()
            .take(levels)
            .map(|(&price, orders)| DepthLevel {
                price,
                total_quantity: orders.iter().map(|order| order.quantity).sum(),
                order_count: orders.len(),
            })
            .collect()
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

    #[test]
    fn spread_returns_best_ask_minus_best_bid() {
        let mut order_book = OrderBook::new();

        order_book.add_order(create_order(1, Side::Buy, 100_000));
        order_book.add_order(create_order(2, Side::Sell, 101_000));

        assert_eq!(order_book.spread(), Some(1_000));
    }

    #[test]
    fn spread_returns_none_when_one_side_is_missing() {
        let mut order_book = OrderBook::new();

        order_book.add_order(create_order(1, Side::Buy, 100_000));

        assert_eq!(order_book.spread(), None);
    }

    fn create_order_with_quantity(id: u64, side: Side, price: i64, quantity: u64) -> Order {
        Order {
            id,
            symbol: String::from("BTCUSDT"),
            side,
            price,
            quantity,
            status: OrderStatus::New,
        }
    }

    #[test]
    fn bid_depth_returns_prices_from_high_to_low() {
        let mut order_book = OrderBook::new();

        order_book.add_order(create_order_with_quantity(1, Side::Buy, 99_000, 5));
        order_book.add_order(create_order_with_quantity(2, Side::Buy, 100_000, 1));
        order_book.add_order(create_order_with_quantity(3, Side::Buy, 100_000, 2));

        assert_eq!(
            order_book.bid_depth(2),
            vec![
                DepthLevel {
                    price: 100_000,
                    total_quantity: 3,
                    order_count: 2,
                },
                DepthLevel {
                    price: 99_000,
                    total_quantity: 5,
                    order_count: 1,
                },
            ]
        );
    }

    #[test]
    fn ask_depth_returns_prices_from_low_to_high() {
        let mut order_book = OrderBook::new();

        order_book.add_order(create_order_with_quantity(1, Side::Sell, 102_000, 4));
        order_book.add_order(create_order_with_quantity(2, Side::Sell, 101_000, 3));

        assert_eq!(
            order_book.ask_depth(2),
            vec![
                DepthLevel {
                    price: 101_000,
                    total_quantity: 3,
                    order_count: 1,
                },
                DepthLevel {
                    price: 102_000,
                    total_quantity: 4,
                    order_count: 1,
                },
            ]
        );
    }
}
