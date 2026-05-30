mod backtest;
mod engine;
mod event;
mod market_data;
mod model;
mod order_book;
mod strategy;
mod tasks;

use event::Event;
use market_data::MarketDataSimulator;
use model::{OrderRequest, Tick};
use strategy::DemoCrossStrategy;
use tasks::{event_logger_task, execution_task, market_data_task, strategy_task};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tick_tx, tick_rx) = mpsc::channel::<Tick>(100);
    let (order_tx, order_rx) = mpsc::channel::<OrderRequest>(100);
    let (event_tx, event_rx) = mpsc::channel::<Event>(100);

    let simulator = MarketDataSimulator::from_csv_path("sample_data/ticks.csv")
        .expect("failed to load sample_data/ticks.csv");

    let strategy = DemoCrossStrategy::new();

    let market_data_handle = tokio::spawn(market_data_task(simulator, tick_tx));
    let strategy_handle = tokio::spawn(strategy_task(strategy, tick_rx, order_tx));
    let execution_handle = tokio::spawn(execution_task(order_rx, event_tx));
    let event_logger_handle = tokio::spawn(event_logger_task(event_rx));

    market_data_handle.await.unwrap();
    strategy_handle.await.unwrap();
    execution_handle.await.unwrap();
    event_logger_handle.await.unwrap();
}
