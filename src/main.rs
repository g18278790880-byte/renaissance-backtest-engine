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

    match order_book.get_order(1) {
        Some(order) => println!("found order: {:?}", order),
        None => println!("order not found"),
    }

    match order_book.get_order(999) {
        Some(order) => println!("found order: {:?}", order),
        None => println!("order 999 not found"),
    }
}
