use crate::model::Tick;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct MarketDataSimulator {
    ticks: Vec<Tick>,
}

impl MarketDataSimulator {
    pub fn new(ticks: Vec<Tick>) -> Self {
        Self { ticks }
    }

    pub fn demo_cross_ticks() -> Self {
        Self::new(vec![
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
        ])
    }

    pub async fn run(self, tick_tx: mpsc::Sender<Tick>) {
        for tick in self.ticks {
            tick_tx.send(tick).await.unwrap();
        }

        println!("market data simulator: all ticks sent");
    }
}
