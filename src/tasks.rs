use crate::engine::Engine;
use crate::event::Event;
use crate::market_data::MarketDataSimulator;
use crate::model::{OrderRequest, Tick};
use crate::strategy::Strategy;
use tokio::sync::mpsc;

pub async fn market_data_task(simulator: MarketDataSimulator, tick_tx: mpsc::Sender<Tick>) {
    simulator.run(tick_tx).await;
}

pub async fn strategy_task<S>(
    mut strategy: S,
    mut tick_rx: mpsc::Receiver<Tick>,
    order_tx: mpsc::Sender<OrderRequest>,
) where
    S: Strategy + Send + 'static,
{
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::OrderStatus;
    use crate::strategy::DemoCrossStrategy;
    use tokio::time::{timeout, Duration};

    async fn recv_event(event_rx: &mut mpsc::Receiver<Event>) -> Event {
        timeout(Duration::from_secs(1), event_rx.recv())
            .await
            .expect("timed out waiting for event")
            .expect("event channel closed before receiving expected event")
    }

    #[tokio::test]
    async fn async_pipeline_generates_trade_and_order_updates() {
        let (tick_tx, tick_rx) = mpsc::channel::<Tick>(10);
        let (order_tx, order_rx) = mpsc::channel::<OrderRequest>(10);
        let (event_tx, mut event_rx) = mpsc::channel::<Event>(10);

        let simulator = MarketDataSimulator::demo_cross_ticks();
        let market_data_handle = tokio::spawn(market_data_task(simulator, tick_tx));
        let strategy = DemoCrossStrategy::new();
        let strategy_handle = tokio::spawn(strategy_task(strategy, tick_rx, order_tx));
        let execution_handle = tokio::spawn(execution_task(order_rx, event_tx));

        let first_event = recv_event(&mut event_rx).await;
        let second_event = recv_event(&mut event_rx).await;
        let third_event = recv_event(&mut event_rx).await;
        let fourth_event = recv_event(&mut event_rx).await;
        let fifth_event = recv_event(&mut event_rx).await;

        match first_event {
            Event::OrderUpdate(update) => {
                assert_eq!(update.order_id, 1);
                assert_eq!(update.status, OrderStatus::New);
                assert_eq!(update.filled_quantity, 0);
                assert_eq!(update.remaining_quantity, 2);
                assert_eq!(update.timestamp, 0);
            }
            _ => panic!("expected new buy order update event"),
        }

        match second_event {
            Event::OrderUpdate(update) => {
                assert_eq!(update.order_id, 2);
                assert_eq!(update.status, OrderStatus::New);
                assert_eq!(update.filled_quantity, 0);
                assert_eq!(update.remaining_quantity, 1);
                assert_eq!(update.timestamp, 0);
            }
            _ => panic!("expected new sell order update event"),
        }

        match third_event {
            Event::Trade(trade) => {
                assert_eq!(trade.trade_id, 1);
                assert_eq!(trade.buy_order_id, 1);
                assert_eq!(trade.sell_order_id, 2);
                assert_eq!(trade.symbol, "BTCUSDT");
                assert_eq!(trade.price, 99_000);
                assert_eq!(trade.quantity, 1);
            }
            _ => panic!("expected trade event"),
        }

        match fourth_event {
            Event::OrderUpdate(update) => {
                assert_eq!(update.order_id, 1);
                assert_eq!(update.status, OrderStatus::PartiallyFilled);
                assert_eq!(update.filled_quantity, 1);
                assert_eq!(update.remaining_quantity, 1);
                assert_eq!(update.timestamp, 0);
            }
            _ => panic!("expected buy order update event"),
        }

        match fifth_event {
            Event::OrderUpdate(update) => {
                assert_eq!(update.order_id, 2);
                assert_eq!(update.status, OrderStatus::Filled);
                assert_eq!(update.filled_quantity, 1);
                assert_eq!(update.remaining_quantity, 0);
                assert_eq!(update.timestamp, 0);
            }
            _ => panic!("expected sell order update event"),
        }

        market_data_handle.await.unwrap();
        strategy_handle.await.unwrap();
        execution_handle.await.unwrap();

        let no_more_events = timeout(Duration::from_secs(1), event_rx.recv())
            .await
            .expect("timed out waiting for event channel to close");

        assert!(no_more_events.is_none());
    }
}
