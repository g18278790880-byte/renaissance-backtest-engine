mod engine;
mod event;
mod model;
mod order_book;
mod strategy;

use engine::Engine;
use event::Event;
use model::{OrderRequest, Side, Tick};
use strategy::Strategy;
use tokio::sync::mpsc;

struct CrossStrategy {
    call_count: usize,
}

impl CrossStrategy {
    fn new() -> Self {
        Self { call_count: 0 }
    }
}

impl Strategy for CrossStrategy {
    fn on_tick(&mut self, tick: &Tick) -> Vec<OrderRequest> {
        self.call_count += 1;

        if self.call_count == 1 {
            vec![OrderRequest {
                symbol: tick.symbol.clone(),
                side: Side::Buy,
                price: 100_000,
                quantity: 2,
            }]
        } else if self.call_count == 2 {
            vec![OrderRequest {
                symbol: tick.symbol.clone(),
                side: Side::Sell,
                price: 99_000,
                quantity: 1,
            }]
        } else {
            Vec::new()
        }
    }
}

#[tokio::main]
async fn main() {
    let (tick_tx, mut tick_rx) = mpsc::channel::<Tick>(100);
    let (order_tx, mut order_rx) = mpsc::channel::<OrderRequest>(100);

    let market_data_task = tokio::spawn(async move {
        let ticks = vec![
            Tick {
                symbol: String::from("BTCUSDT"),
                price: 100_000,
                quantity: 1,
                timestamp: 1,
            },
            Tick {
                symbol: String::from("BTCUSDT"),
                price: 99_000,
                quantity: 1,
                timestamp: 2,
            },
        ];

        for tick in ticks {
            tick_tx.send(tick).await.unwrap();
        }

        println!("market data task: all ticks sent");
    });

    let strategy_task = tokio::spawn(async move {
        let mut strategy = CrossStrategy::new();

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

    let execution_task = tokio::spawn(async move {
        let mut engine = Engine::new();

        while let Some(request) = order_rx.recv().await {
            println!("execution task: order request received: {:?}", request);

            match engine.handle_event(Event::OrderRequest(request)) {
                Ok(output_events) => {
                    if output_events.is_empty() {
                        println!("execution task: no trade generated");
                    } else {
                        for event in output_events {
                            println!("execution task: output event: {:?}", event);
                        }
                    }

                    println!("execution task: order count = {}", engine.order_count());
                    println!("execution task: best bid = {:?}", engine.best_bid());
                    println!("execution task: best ask = {:?}", engine.best_ask());
                }
                Err(err) => {
                    println!("execution task: failed to handle order request: {:?}", err);
                }
            }
        }

        println!("execution task: order request channel closed");
    });

    market_data_task.await.unwrap();
    strategy_task.await.unwrap();
    execution_task.await.unwrap();
}
