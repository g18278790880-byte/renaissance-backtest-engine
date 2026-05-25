mod model;

use model::{Order, Side};

fn main() {
    let order = Order {
        id: 1,
        symbol: String::from("BTCUSDT"),
        side: Side::Buy,
        price: 100_000,
        quantity: 1,
    };

    println!("{}", order.symbol);
}
