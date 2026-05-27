mod engine;
mod event;
mod model;
mod order_book;

use engine::Engine;
use event::Event;
use model::{OrderRequest, Side};

fn main() {
    let mut engine = Engine::new();

    let buy_event = Event::OrderRequest(OrderRequest {
        symbol: String::from("BTCUSDT"),
        side: Side::Buy,
        price: 100_000,
        quantity: 2,
    });

    let sell_event = Event::OrderRequest(OrderRequest {
        symbol: String::from("BTCUSDT"),
        side: Side::Sell,
        price: 99_000,
        quantity: 1,
    });

    let output_events = engine.handle_event(buy_event).unwrap();
    println!("after buy event outputs: {:?}", output_events);
    println!("order count: {}", engine.order_count());
    println!("best bid: {:?}", engine.best_bid());
    println!("best ask: {:?}", engine.best_ask());

    let output_events = engine.handle_event(sell_event).unwrap();
    println!("after sell event outputs: {:?}", output_events);
    println!("order count: {}", engine.order_count());
    println!("best bid: {:?}", engine.best_bid());
    println!("best ask: {:?}", engine.best_ask());
}
