mod model;

use model::{Order, Side, Trade};

fn main() {
    let order = Order {
        id: 1,
        symbol: String::from("BTCUSDT"),
        side: Side::Buy,
        price: 100_000,
        quantity: 1,
    };

    println!("new order: {}", order.symbol);

    let trade = Trade {
        trade_id: 1,
        order_id: order.id,
        symbol: order.symbol.clone(),
        price: 100_000,
        quantity: 1,
        timestamp: 1_717_000_000,
    };

    println!("trade executed: {:?}", trade);
}
