mod model;
mod order_book;

use model::{Order, OrderStatus, Side};
use order_book::OrderBook;

fn main() {
    let mut order_book = OrderBook::new();

    let order1 = Order {
        id: 1,
        symbol: String::from("BTCUSDT"),
        side: Side::Buy,
        price: 100_000,
        quantity: 1,
        status: OrderStatus::New,
    };

    let order2 = Order {
        id: 2,
        symbol: String::from("BTCUSDT"),
        side: Side::Buy,
        price: 100_000,
        quantity: 2,
        status: OrderStatus::New,
    };

    order_book.add_order(order1);
    order_book.add_order(order2);

    println!("{:#?}", order_book);
}
