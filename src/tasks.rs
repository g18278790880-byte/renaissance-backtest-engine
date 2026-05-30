use crate::engine::Engine;
use crate::event::Event;
use crate::model::{OrderRequest, Side, Tick};
use crate::strategy::Strategy;
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

pub async fn market_data_task(tick_tx: mpsc::Sender<Tick>) {
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
}

pub async fn strategy_task(
    mut tick_rx: mpsc::Receiver<Tick>,
    order_tx: mpsc::Sender<OrderRequest>,
) {
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
}

pub async fn execution_task(
    mut order_rx: mpsc::Receiver<OrderRequest>,
    event_tx: mpsc::Sender<Event>,
) {
    let mut engine = Engine::new();

    while let Some(request) = order_rx.recv().await {
        println!("execution task: order request received: {:?}", request);

        match engine.handle_event(Event::OrderRequest(request)) {
            Ok(output_events) => {
                if output_events.is_empty() {
                    println!("execution task: no output event generated");
                }

                for event in output_events {
                    event_tx.send(event).await.unwrap();
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
}

pub async fn event_logger_task(mut event_rx: mpsc::Receiver<Event>) {
    while let Some(event) = event_rx.recv().await {
        println!("event logger task: event received: {:?}", event);
    }

    println!("event logger task: event channel closed");
}
