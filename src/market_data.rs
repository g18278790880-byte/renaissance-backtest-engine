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

    pub fn len(&self) -> usize {
        self.ticks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ticks.is_empty()
    }

    pub async fn run(self, tick_tx: mpsc::Sender<Tick>) {
        for tick in self.ticks {
            tick_tx.send(tick).await.unwrap();
        }

        println!("market data simulator: all ticks sent");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tick(price: i64, timestamp: u64) -> Tick {
        Tick {
            symbol: String::from("BTCUSDT"),
            price,
            quantity: 1,
            timestamp,
        }
    }

    #[test]
    fn new_creates_market_data_simulator_with_ticks() {
        let simulator =
            MarketDataSimulator::new(vec![create_tick(100_000, 1), create_tick(99_000, 2)]);

        assert_eq!(simulator.len(), 2);
        assert!(!simulator.is_empty());
    }

    #[test]
    fn demo_cross_ticks_contains_two_ticks() {
        let simulator = MarketDataSimulator::demo_cross_ticks();

        assert_eq!(simulator.len(), 2);
        assert!(!simulator.is_empty());
    }

    #[tokio::test]
    async fn run_sends_all_ticks_to_channel() {
        let simulator =
            MarketDataSimulator::new(vec![create_tick(100_000, 1), create_tick(99_000, 2)]);

        let (tick_tx, mut tick_rx) = mpsc::channel::<Tick>(10);

        simulator.run(tick_tx).await;

        let first_tick = tick_rx.recv().await.unwrap();
        let second_tick = tick_rx.recv().await.unwrap();
        let third_tick = tick_rx.recv().await;

        assert_eq!(first_tick.price, 100_000);
        assert_eq!(first_tick.timestamp, 1);

        assert_eq!(second_tick.price, 99_000);
        assert_eq!(second_tick.timestamp, 2);

        assert!(third_tick.is_none());
    }
}
