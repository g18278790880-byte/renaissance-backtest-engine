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
            quantity: 3,
            status: OrderStatus::New,
        })
        .unwrap();

    order_book
        .add_order(Order {
            id: 2,
            symbol: String::from("BTCUSDT"),
            side: Side::Sell,
            price: 99_000,
            quantity: 2,
            status: OrderStatus::New,
        })
        .unwrap();

    let trade = order_book.match_best_orders(1, 1_717_000_000);

    println!("trade: {:?}", trade);
    println!("order count: {}", order_book.order_count());
    println!("buy order after match: {:?}", order_book.get_order(1));
    println!("sell order after match: {:?}", order_book.get_order(2));
}
