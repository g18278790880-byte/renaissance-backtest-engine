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
            side: Side::Sell,
            price: 101_000,
            quantity: 2,
            status: OrderStatus::New,
        })
        .unwrap();

    println!("order count: {}", order_book.order_count());
    println!("bid order count: {}", order_book.bid_order_count());
    println!("ask order count: {}", order_book.ask_order_count());

    order_book.cancel_order(1).unwrap();

    println!("after cancel order count: {}", order_book.order_count());
    println!(
        "after cancel bid order count: {}",
        order_book.bid_order_count()
    );
}
