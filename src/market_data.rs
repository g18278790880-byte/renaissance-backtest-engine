use crate::model::Tick;
use std::fs;
use std::path::Path;
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

    pub fn from_csv_path(path: impl AsRef<Path>) -> Result<Self, MarketDataError> {
        let content = fs::read_to_string(path).map_err(|_| MarketDataError::Io)?;

        Self::from_csv_str(&content)
    }

    fn from_csv_str(content: &str) -> Result<Self, MarketDataError> {
        let mut lines = content.lines();

        let header = lines.next().ok_or(MarketDataError::InvalidHeader)?;

        if header.trim() != "timestamp,symbol,price,quantity" {
            return Err(MarketDataError::InvalidHeader);
        }

        let mut ticks = Vec::new();

        for (index, line) in lines.enumerate() {
            let line_number = index + 2;
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let columns: Vec<&str> = line.split(',').map(str::trim).collect();

            if columns.len() != 4 {
                return Err(MarketDataError::InvalidColumnCount {
                    line: line_number,
                    actual: columns.len(),
                });
            }

            let timestamp = columns[0]
                .parse::<u64>()
                .map_err(|_| MarketDataError::InvalidTimestamp { line: line_number })?;

            let symbol = columns[1].to_string();

            let price = columns[2]
                .parse::<i64>()
                .map_err(|_| MarketDataError::InvalidPrice { line: line_number })?;

            let quantity = columns[3]
                .parse::<u64>()
                .map_err(|_| MarketDataError::InvalidQuantity { line: line_number })?;

            ticks.push(Tick {
                symbol,
                price,
                quantity,
                timestamp,
            });
        }

        Ok(Self::new(ticks))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MarketDataError {
    Io,
    InvalidHeader,
    InvalidColumnCount { line: usize, actual: usize },
    InvalidTimestamp { line: usize },
    InvalidPrice { line: usize },
    InvalidQuantity { line: usize },
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

    #[test]
    fn from_csv_str_loads_ticks() {
        let content = "\
timestamp,symbol,price,quantity
1,BTCUSDT,100000,1
2,BTCUSDT,99000,2
";

        let simulator = MarketDataSimulator::from_csv_str(content).unwrap();

        assert_eq!(simulator.len(), 2);
        assert!(!simulator.is_empty());
    }

    #[test]
    fn from_csv_str_rejects_invalid_header() {
        let content = "\
time,symbol,price,quantity
1,BTCUSDT,100000,1
";

        let result = MarketDataSimulator::from_csv_str(content);

        assert_eq!(result.err(), Some(MarketDataError::InvalidHeader));
    }

    #[test]
    fn from_csv_str_rejects_invalid_price() {
        let content = "\
timestamp,symbol,price,quantity
1,BTCUSDT,not_a_price,1
";

        let result = MarketDataSimulator::from_csv_str(content);

        assert_eq!(
            result.err(),
            Some(MarketDataError::InvalidPrice { line: 2 })
        );
    }
}
