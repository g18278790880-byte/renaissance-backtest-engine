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

    println!("contains order 1: {}", order_book.contains_order(1));
    println!("contains order 999: {}", order_book.contains_order(999));

    order_book.cancel_order(1).unwrap();

    println!(
        "contains order 1 after cancel: {}",
        order_book.contains_order(1)
    );
}
