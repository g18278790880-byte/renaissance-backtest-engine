mod model;

use model::{Order, OrderStatus, Side};

fn main() {
    let mut order = Order {
        id: 1,
        symbol: String::from("BTCUSDT"),
        side: Side::Buy,
        price: 100_000,
        quantity: 1,
        status: OrderStatus::PartiallyFilled,
    };

    println!("new order: {:?}", order);

    order.cancel();

    println!("after cancel: {:?}", order);
}
