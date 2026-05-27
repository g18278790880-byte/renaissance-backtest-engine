mod event;
mod model;
mod order_book;

use event::Event;
use model::{OrderRequest, Side, Tick};

fn main() {
    let events = vec![
        Event::MarketTick(Tick {
            symbol: String::from("BTCUSDT"),
            price: 100_000,
            quantity: 1,
            timestamp: 1_717_000_000,
        }),
        Event::OrderRequest(OrderRequest {
            symbol: String::from("BTCUSDT"),
            side: Side::Buy,
            price: 99_000,
            quantity: 1,
        }),
    ];

    for event in &events {
        println!("event type: {}", event.event_type());
        println!("event detail: {:?}", event);
    }
}
