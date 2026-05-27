mod model;
mod order_book;

use model::{Order, OrderStatus, Side};
use order_book::OrderBook;

fn main() {
    let mut order_book = OrderBook::new();

    order_book
        .add_order(Order {
            id: 1,
            symbol: String::from("BTCUSDT"),
            side: Side::Buy,
            price: 100_000,
            quantity: 1,
            status: OrderStatus::New,
        })
        .unwrap();

    order_book
        .add_order(Order {
            id: 2,
            symbol: String::from("BTCUSDT"),
            side: Side::Buy,
            price: 100_000,
            quantity: 2,
            status: OrderStatus::New,
        })
        .unwrap();

    order_book
        .add_order(Order {
            id: 3,
            symbol: String::from("BTCUSDT"),
            side: Side::Sell,
            price: 101_000,
            quantity: 1,
            status: OrderStatus::New,
        })
        .unwrap();

    match order_book.best_bid_order() {
        Some(order) => println!("best bid order: {:?}", order),
        None => println!("no bid order"),
    }

    match order_book.best_ask_order() {
        Some(order) => println!("best ask order: {:?}", order),
        None => println!("no ask order"),
    }
}
