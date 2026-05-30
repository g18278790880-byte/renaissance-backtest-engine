use crate::model::Side;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub symbol: String,
    pub quantity: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Portfolio {
    cash: i128,
    positions: HashMap<String, Position>,
    trade_count: usize,
}

impl Portfolio {
    pub fn new() -> Self {
        Self {
            cash: 0,
            positions: HashMap::new(),
            trade_count: 0,
        }
    }

    pub fn apply_fill(&mut self, symbol: &str, side: Side, price: i64, quantity: u64) {
        let quantity_i64 = quantity as i64;
        let notional = price as i128 * quantity as i128;

        let position = self
            .positions
            .entry(symbol.to_string())
            .or_insert(Position {
                symbol: symbol.to_string(),
                quantity: 0,
            });

        match side {
            Side::Buy => {
                position.quantity += quantity_i64;
                self.cash -= notional;
            }
            Side::Sell => {
                position.quantity -= quantity_i64;
                self.cash += notional;
            }
        }

        self.trade_count += 1;
    }

    pub fn cash(&self) -> i128 {
        self.cash
    }

    pub fn trade_count(&self) -> usize {
        self.trade_count
    }

    pub fn position_quantity(&self, symbol: &str) -> i64 {
        self.positions
            .get(symbol)
            .map(|position| position.quantity)
            .unwrap_or(0)
    }

    pub fn position_quantities(&self) -> HashMap<String, i64> {
        self.positions
            .iter()
            .map(|(symbol, position)| (symbol.clone(), position.quantity))
            .collect()
    }

    pub fn equity(&self, last_prices: &HashMap<String, i64>) -> i128 {
        let position_value: i128 = self
            .positions
            .iter()
            .map(|(symbol, position)| {
                let last_price = last_prices.get(symbol).copied().unwrap_or(0);
                position.quantity as i128 * last_price as i128
            })
            .sum();

        self.cash + position_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portfolio_apply_buy_fill_increases_position_and_decreases_cash() {
        let mut portfolio = Portfolio::new();

        portfolio.apply_fill("BTCUSDT", Side::Buy, 100_000, 2);

        assert_eq!(portfolio.position_quantity("BTCUSDT"), 2);
        assert_eq!(portfolio.cash(), -200_000);
        assert_eq!(portfolio.trade_count(), 1);
    }

    #[test]
    fn portfolio_apply_sell_fill_decreases_position_and_increases_cash() {
        let mut portfolio = Portfolio::new();

        portfolio.apply_fill("BTCUSDT", Side::Sell, 100_000, 2);

        assert_eq!(portfolio.position_quantity("BTCUSDT"), -2);
        assert_eq!(portfolio.cash(), 200_000);
        assert_eq!(portfolio.trade_count(), 1);
    }

    #[test]
    fn portfolio_tracks_multiple_symbols() {
        let mut portfolio = Portfolio::new();

        portfolio.apply_fill("BTCUSDT", Side::Buy, 100_000, 2);
        portfolio.apply_fill("ETHUSDT", Side::Sell, 3_000, 1);

        assert_eq!(portfolio.position_quantity("BTCUSDT"), 2);
        assert_eq!(portfolio.position_quantity("ETHUSDT"), -1);
        assert_eq!(portfolio.cash(), -197_000);
        assert_eq!(portfolio.trade_count(), 2);
    }

    #[test]
    fn portfolio_returns_zero_for_missing_position() {
        let portfolio = Portfolio::new();

        assert_eq!(portfolio.position_quantity("BTCUSDT"), 0);
    }

    #[test]
    fn portfolio_equity_marks_position_to_last_price() {
        let mut portfolio = Portfolio::new();

        portfolio.apply_fill("BTCUSDT", Side::Buy, 100_000, 1);

        let mut last_prices = HashMap::new();
        last_prices.insert("BTCUSDT".to_string(), 105_000);

        assert_eq!(portfolio.cash(), -100_000);
        assert_eq!(portfolio.position_quantity("BTCUSDT"), 1);
        assert_eq!(portfolio.equity(&last_prices), 5_000);
    }

    #[test]
    fn portfolio_equity_marks_short_position_to_last_price() {
        let mut portfolio = Portfolio::new();

        portfolio.apply_fill("BTCUSDT", Side::Sell, 100_000, 1);

        let mut last_prices = HashMap::new();
        last_prices.insert("BTCUSDT".to_string(), 95_000);

        assert_eq!(portfolio.cash(), 100_000);
        assert_eq!(portfolio.position_quantity("BTCUSDT"), -1);
        assert_eq!(portfolio.equity(&last_prices), 5_000);
    }
}
