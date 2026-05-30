mod engine;
mod event;
mod model;
mod order_book;
mod strategy;

use model::Tick;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tick_tx, mut tick_rx) = mpsc::channel::<Tick>(100);

    let market_data_task = tokio::spawn(async move {
        let tick = Tick {
            symbol: String::from("BTCUSDT"),
            price: 98_000,
            quantity: 1,
            timestamp: 1,
        };

        tick_tx.send(tick).await.unwrap();

        println!("market data task: tick sent");
    });

    let strategy_task = tokio::spawn(async move {
        while let Some(tick) = tick_rx.recv().await {
            println!("strategy task: tick received: {:?}", tick);
        }
    });

    market_data_task.await.unwrap();
    strategy_task.await.unwrap();
}
