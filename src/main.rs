mod engine;
mod event;
mod model;
mod order_book;
mod strategy;

use engine::Engine;
use model::Tick;
use strategy::ThresholdStrategy;

fn main() {
    let mut engine = Engine::new();

    let mut strategy = ThresholdStrategy::new(String::from("BTCUSDT"), 99_000, 101_000, 1);

    let tick = Tick {
        symbol: String::from("BTCUSDT"),
        price: 98_000,
        quantity: 1,
        timestamp: 1_717_000_000,
    };

    let output_events = engine.process_market_tick(&tick, &mut strategy).unwrap();

    println!("output events: {:?}", output_events);
    println!("order count: {}", engine.order_count());
    println!("best bid: {:?}", engine.best_bid());
    println!("best ask: {:?}", engine.best_ask());
}
