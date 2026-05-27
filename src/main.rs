mod model;
mod order_book;

use model::{Order, OrderStatus, Side};
use order_book::OrderBook;

fn main() {
    let mut order_book = OrderBook::new();

    let buy_order = Order {
        id: 1,
        symbol: String::from("BTCUSDT"),
        side: Side::Buy,
        price: 100_000,
        quantity: 1,
        status: OrderStatus::New,
    };

    let sell_order = Order {
        id: 2,
        symbol: String::from("BTCUSDT"),
        side: Side::Sell,
        price: 101_000,
        quantity: 1,
        status: OrderStatus::New,
    };

    order_book.add_order(buy_order);
    order_book.add_order(sell_order);

    println!("best bid: {:?}", order_book.best_bid());
    println!("best ask: {:?}", order_book.best_ask());
    println!("{:#?}", order_book);
}
