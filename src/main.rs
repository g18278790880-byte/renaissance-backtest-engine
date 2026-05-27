mod model;
mod order_book;

use model::{OrderRequest, Side};
use order_book::OrderBook;

fn main() {
    let mut order_book = OrderBook::new();

    let request = OrderRequest {
        symbol: String::from("BTCUSDT"),
        side: Side::Buy,
        price: 100_000,
        quantity: 1,
    };

    let order = request.into_order(1);

    order_book.add_order(order).unwrap();

    println!("order count: {}", order_book.order_count());
    println!("best bid: {:?}", order_book.best_bid());
}
