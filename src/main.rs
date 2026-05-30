mod engine;
mod event;
mod model;
mod order_book;
mod strategy;

use model::{OrderRequest, Tick};
use strategy::{Strategy, ThresholdStrategy};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tick_tx, mut tick_rx) = mpsc::channel::<Tick>(100);
    let (order_tx, mut order_rx) = mpsc::channel::<OrderRequest>(100);

    let market_data_task = tokio::spawn(async move {
        let ticks = vec![
            Tick {
                symbol: String::from("BTCUSDT"),
                price: 98_000,
                quantity: 1,
                timestamp: 1,
            },
            Tick {
                symbol: String::from("BTCUSDT"),
                price: 100_000,
                quantity: 1,
                timestamp: 2,
            },
            Tick {
                symbol: String::from("BTCUSDT"),
                price: 102_000,
                quantity: 1,
                timestamp: 3,
            },
        ];

        for tick in ticks {
            tick_tx.send(tick).await.unwrap();
        }

        println!("market data task: all ticks sent");
    });

    let strategy_task = tokio::spawn(async move {
        let mut strategy = ThresholdStrategy::new(String::from("BTCUSDT"), 99_000, 101_000, 1);

        while let Some(tick) = tick_rx.recv().await {
            println!("strategy task: tick received: {:?}", tick);

            let requests = strategy.on_tick(&tick);

            for request in requests {
                println!("strategy task: order request generated: {:?}", request);
                order_tx.send(request).await.unwrap();
            }
        }

        println!("strategy task: tick channel closed");
    });

    let order_logger_task = tokio::spawn(async move {
        while let Some(request) = order_rx.recv().await {
            println!("order logger task: order request received: {:?}", request);
        }

        println!("order logger task: order request channel closed");
    });

    market_data_task.await.unwrap();
    strategy_task.await.unwrap();
    order_logger_task.await.unwrap();
}
