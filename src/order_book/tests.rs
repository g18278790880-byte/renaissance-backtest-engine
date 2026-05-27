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

    order_book
        .add_order(create_order(1, Side::Buy, 99_000))
        .unwrap();
    order_book
        .add_order(create_order(2, Side::Buy, 100_000))
        .unwrap();

    assert_eq!(order_book.best_bid(), Some(100_000));
}

#[test]
fn best_ask_returns_lowest_sell_price() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order(1, Side::Sell, 102_000))
        .unwrap();
    order_book
        .add_order(create_order(2, Side::Sell, 101_000))
        .unwrap();

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

    order_book
        .add_order(create_order(1, Side::Buy, 100_000))
        .unwrap();
    order_book
        .add_order(create_order(2, Side::Sell, 101_000))
        .unwrap();

    assert_eq!(order_book.spread(), Some(1_000));
}

#[test]
fn spread_returns_none_when_one_side_is_missing() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order(1, Side::Buy, 100_000))
        .unwrap();

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

    order_book
        .add_order(create_order_with_quantity(1, Side::Buy, 99_000, 5))
        .unwrap();
    order_book
        .add_order(create_order_with_quantity(2, Side::Buy, 100_000, 1))
        .unwrap();
    order_book
        .add_order(create_order_with_quantity(3, Side::Buy, 100_000, 2))
        .unwrap();

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

    order_book
        .add_order(create_order_with_quantity(1, Side::Sell, 102_000, 4))
        .unwrap();
    order_book
        .add_order(create_order_with_quantity(2, Side::Sell, 101_000, 3))
        .unwrap();

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

#[test]
fn cancel_order_removes_order_from_order_book() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order(1, Side::Buy, 100_000))
        .unwrap();
    order_book
        .add_order(create_order(2, Side::Buy, 99_000))
        .unwrap();

    let result = order_book.cancel_order(1);

    assert!(result.is_ok());
    assert_eq!(order_book.best_bid(), Some(99_000));
}

#[test]
fn cancel_order_returns_error_when_order_does_not_exist() {
    let mut order_book = OrderBook::new();

    let result = order_book.cancel_order(999);

    assert_eq!(result, Err(OrderBookError::OrderNotFound(999)));
}

#[test]
fn add_order_rejects_duplicate_order_id() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order(1, Side::Buy, 100_000))
        .unwrap();

    let result = order_book.add_order(create_order(1, Side::Buy, 99_000));

    assert_eq!(result, Err(OrderBookError::DuplicateOrderId(1)));
}

#[test]
fn order_count_returns_active_order_count() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order(1, Side::Buy, 100_000))
        .unwrap();

    order_book
        .add_order(create_order(2, Side::Sell, 101_000))
        .unwrap();

    assert_eq!(order_book.order_count(), 2);
    assert_eq!(order_book.bid_order_count(), 1);
    assert_eq!(order_book.ask_order_count(), 1);
}

#[test]
fn order_count_decreases_after_cancel_order() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order(1, Side::Buy, 100_000))
        .unwrap();

    order_book
        .add_order(create_order(2, Side::Buy, 99_000))
        .unwrap();

    order_book.cancel_order(1).unwrap();

    assert_eq!(order_book.order_count(), 1);
    assert_eq!(order_book.bid_order_count(), 1);
}

#[test]
fn contains_order_returns_true_when_order_exists() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order(1, Side::Buy, 100_000))
        .unwrap();

    assert!(order_book.contains_order(1));
}

#[test]
fn contains_order_returns_false_when_order_does_not_exist() {
    let order_book = OrderBook::new();

    assert!(!order_book.contains_order(999));
}

#[test]
fn contains_order_returns_false_after_order_is_cancelled() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order(1, Side::Buy, 100_000))
        .unwrap();

    order_book.cancel_order(1).unwrap();

    assert!(!order_book.contains_order(1));
}

#[test]
fn get_order_returns_order_when_order_exists() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order(1, Side::Buy, 100_000))
        .unwrap();

    let order = order_book.get_order(1);

    assert!(order.is_some());
    assert_eq!(order.unwrap().id, 1);
}

#[test]
fn get_order_returns_none_when_order_does_not_exist() {
    let order_book = OrderBook::new();

    assert!(order_book.get_order(999).is_none());
}

#[test]
fn get_order_returns_none_after_order_is_cancelled() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order(1, Side::Buy, 100_000))
        .unwrap();

    order_book.cancel_order(1).unwrap();

    assert!(order_book.get_order(1).is_none());
}

#[test]
fn best_bid_order_returns_first_order_at_highest_bid_price() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order(1, Side::Buy, 100_000))
        .unwrap();

    order_book
        .add_order(create_order(2, Side::Buy, 100_000))
        .unwrap();

    order_book
        .add_order(create_order(3, Side::Buy, 99_000))
        .unwrap();

    let best_order = order_book.best_bid_order().unwrap();

    assert_eq!(best_order.id, 1);
}

#[test]
fn best_ask_order_returns_first_order_at_lowest_ask_price() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order(1, Side::Sell, 102_000))
        .unwrap();

    order_book
        .add_order(create_order(2, Side::Sell, 101_000))
        .unwrap();

    order_book
        .add_order(create_order(3, Side::Sell, 101_000))
        .unwrap();

    let best_order = order_book.best_ask_order().unwrap();

    assert_eq!(best_order.id, 2);
}

#[test]
fn best_bid_order_returns_none_when_no_bid_exists() {
    let order_book = OrderBook::new();

    assert!(order_book.best_bid_order().is_none());
}

#[test]
fn best_ask_order_returns_none_when_no_ask_exists() {
    let order_book = OrderBook::new();

    assert!(order_book.best_ask_order().is_none());
}

#[test]
fn match_best_orders_creates_trade_when_prices_cross() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order_with_quantity(1, Side::Buy, 100_000, 3))
        .unwrap();

    order_book
        .add_order(create_order_with_quantity(2, Side::Sell, 99_000, 2))
        .unwrap();

    let trade = order_book.match_best_orders(1, 1_717_000_000).unwrap();

    assert_eq!(trade.buy_order_id, 1);
    assert_eq!(trade.sell_order_id, 2);
    assert_eq!(trade.price, 99_000);
    assert_eq!(trade.quantity, 2);

    assert!(order_book.contains_order(1));
    assert!(!order_book.contains_order(2));

    let buy_order = order_book.get_order(1).unwrap();

    assert_eq!(buy_order.quantity, 1);
    assert_eq!(buy_order.status, OrderStatus::PartiallyFilled);
}

#[test]
fn match_best_orders_removes_both_orders_when_fully_matched() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order_with_quantity(1, Side::Buy, 100_000, 2))
        .unwrap();

    order_book
        .add_order(create_order_with_quantity(2, Side::Sell, 99_000, 2))
        .unwrap();

    let trade = order_book.match_best_orders(1, 1_717_000_000).unwrap();

    assert_eq!(trade.quantity, 2);
    assert_eq!(order_book.order_count(), 0);
    assert!(order_book.best_bid().is_none());
    assert!(order_book.best_ask().is_none());
}

#[test]
fn match_best_orders_returns_none_when_prices_do_not_cross() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order_with_quantity(1, Side::Buy, 100_000, 2))
        .unwrap();

    order_book
        .add_order(create_order_with_quantity(2, Side::Sell, 101_000, 2))
        .unwrap();

    let trade = order_book.match_best_orders(1, 1_717_000_000);

    assert!(trade.is_none());
    assert_eq!(order_book.order_count(), 2);
    assert!(order_book.contains_order(1));
    assert!(order_book.contains_order(2));
}

#[test]
fn match_orders_continues_until_prices_do_not_cross() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order_with_quantity(1, Side::Buy, 100_000, 5))
        .unwrap();

    order_book
        .add_order(create_order_with_quantity(2, Side::Sell, 99_000, 2))
        .unwrap();

    order_book
        .add_order(create_order_with_quantity(3, Side::Sell, 99_500, 2))
        .unwrap();

    order_book
        .add_order(create_order_with_quantity(4, Side::Sell, 100_000, 1))
        .unwrap();

    order_book
        .add_order(create_order_with_quantity(5, Side::Sell, 101_000, 1))
        .unwrap();

    let trades = order_book.match_orders(1, 1_717_000_000);

    assert_eq!(trades.len(), 3);

    assert_eq!(trades[0].sell_order_id, 2);
    assert_eq!(trades[0].quantity, 2);

    assert_eq!(trades[1].sell_order_id, 3);
    assert_eq!(trades[1].quantity, 2);

    assert_eq!(trades[2].sell_order_id, 4);
    assert_eq!(trades[2].quantity, 1);

    assert!(!order_book.contains_order(1));
    assert!(!order_book.contains_order(2));
    assert!(!order_book.contains_order(3));
    assert!(!order_book.contains_order(4));
    assert!(order_book.contains_order(5));

    assert_eq!(order_book.best_bid(), None);
    assert_eq!(order_book.best_ask(), Some(101_000));
}

#[test]
fn match_orders_returns_empty_vec_when_no_match_exists() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(create_order_with_quantity(1, Side::Buy, 100_000, 2))
        .unwrap();

    order_book
        .add_order(create_order_with_quantity(2, Side::Sell, 101_000, 2))
        .unwrap();

    let trades = order_book.match_orders(1, 1_717_000_000);

    assert!(trades.is_empty());
    assert_eq!(order_book.order_count(), 2);
}
