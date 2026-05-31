use crate::model::Side;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub symbol: String,
    pub quantity: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Portfolio {
    initial_cash: i128,
    cash: i128,
    positions: HashMap<String, Position>,
    trade_count: usize,
    fee_paid: i128,
}

impl Portfolio {
    pub fn new() -> Self {
        Self::with_initial_cash(0)
    }

    pub fn with_initial_cash(initial_cash: i128) -> Self {
        Self {
            initial_cash,
            cash: initial_cash,
            positions: HashMap::new(),
            trade_count: 0,
            fee_paid: 0,
        }
    }

    pub fn initial_cash(&self) -> i128 {
        self.initial_cash
    }

    pub fn apply_fill(&mut self, symbol: &str, side: Side, price: i64, quantity: u64) {
        self.apply_fill_with_fee(symbol, side, price, quantity, 0);
    }

    pub fn apply_fill_with_fee(
        &mut self,
        symbol: &str,
        side: Side,
        price: i64,
        quantity: u64,
        fee: i128,
    ) {
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
                self.cash -= notional + fee;
            }
            Side::Sell => {
                position.quantity -= quantity_i64;
                self.cash += notional - fee;
            }
        }

        self.fee_paid += fee;
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

    pub fn fee_paid(&self) -> i128 {
        self.fee_paid
    }
}

#[cfg(test)]
mod tests;
