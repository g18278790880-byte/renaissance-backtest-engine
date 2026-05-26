mod model;

use model::{Order, OrderStatus, Side};

fn main() {
    let mut orders: Vec<Order> = Vec::new();

    let order = Order {
        id: 1,
        symbol: String::from("BTCUSDT"),
        side: Side::Buy,
        price: 100_000,
        quantity: 1,
        status: OrderStatus::New,
    };

    orders.push(order);

    println!("orders count: {}", orders.len());
    println!("first order: {:?}", orders[0]);
}
