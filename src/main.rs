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
        price: 100_000,
        quantity: 2,
        status: OrderStatus::New,
    });

    order_book.add_order(Order {
        id: 3,
        symbol: String::from("BTCUSDT"),
        side: Side::Buy,
        price: 99_000,
        quantity: 5,
        status: OrderStatus::New,
    });

    order_book.add_order(Order {
        id: 4,
        symbol: String::from("BTCUSDT"),
        side: Side::Sell,
        price: 101_000,
        quantity: 3,
        status: OrderStatus::New,
    });

    order_book.add_order(Order {
        id: 5,
        symbol: String::from("BTCUSDT"),
        side: Side::Sell,
        price: 102_000,
        quantity: 4,
        status: OrderStatus::New,
    });

    println!("best bid: {:?}", order_book.best_bid());
    println!("best ask: {:?}", order_book.best_ask());
    println!("spread: {:?}", order_book.spread());

    println!("bid depth: {:#?}", order_book.bid_depth(5));
    println!("ask depth: {:#?}", order_book.ask_depth(5));
}
