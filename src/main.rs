mod engine;
mod event;
mod model;
mod order_book;
mod strategy;

use model::Tick;
use strategy::{Strategy, ThresholdStrategy};

fn main() {
    let mut strategy = ThresholdStrategy::new(String::from("BTCUSDT"), 99_000, 101_000, 1);

    let tick = Tick {
        symbol: String::from("BTCUSDT"),
        price: 98_000,
        quantity: 1,
        timestamp: 1_717_000_000,
    };

    let requests = strategy.on_tick(&tick);

    println!("generated order requests: {:?}", requests);
}
