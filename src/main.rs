mod model;
mod order_book;

use model::{Order, OrderStatus, Side};
use order_book::OrderBook;

fn main() {
    let mut order_book = OrderBook::new();

    order_book.add_order(Order {
        id: 1,
        symbol: String::from("BTCUSDT"),
        side: Side::Buy,
        price: 100_000,
        quantity: 1,
        status: OrderStatus::New,
    });

    order_book.add_order(Order {
        id: 2,
        symbol: String::from("BTCUSDT"),
        side: Side::Buy,
        price: 99_000,
        quantity: 1,
        status: OrderStatus::New,
    });

    println!("before cancel best bid: {:?}", order_book.best_bid());

    match order_book.cancel_order(1) {
        Ok(order) => println!("cancelled order: {:?}", order),
        Err(err) => println!("cancel failed: {:?}", err),
    }

    println!("after cancel best bid: {:?}", order_book.best_bid());
}
