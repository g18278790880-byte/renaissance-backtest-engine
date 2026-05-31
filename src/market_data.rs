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
mod tests;
