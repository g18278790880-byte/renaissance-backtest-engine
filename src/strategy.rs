use crate::model::{OrderRequest, OrderUpdate, Side, Tick};

pub trait Strategy {
    fn on_tick(&mut self, tick: &Tick) -> Vec<OrderRequest>;

    fn on_order_update(&mut self, _update: &OrderUpdate) {}
}

#[derive(Debug)]
pub struct ThresholdStrategy {
    pub symbol: String,
    pub buy_below: i64,
    pub sell_above: i64,
    pub quantity: u64,
}

impl ThresholdStrategy {
    pub fn new(symbol: String, buy_below: i64, sell_above: i64, quantity: u64) -> Self {
        Self {
            symbol,
            buy_below,
            sell_above,
            quantity,
        }
    }
}

impl Strategy for ThresholdStrategy {
    fn on_tick(&mut self, tick: &Tick) -> Vec<OrderRequest> {
        if tick.symbol != self.symbol {
            return Vec::new();
        }

        if tick.price <= self.buy_below {
            vec![OrderRequest {
                symbol: tick.symbol.clone(),
                side: Side::Buy,
                price: tick.price,
                quantity: self.quantity,
            }]
        } else if tick.price >= self.sell_above {
            vec![OrderRequest {
                symbol: tick.symbol.clone(),
                side: Side::Sell,
                price: tick.price,
                quantity: self.quantity,
            }]
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct DemoCrossStrategy {
    call_count: usize,
}

impl DemoCrossStrategy {
    pub fn new() -> Self {
        Self { call_count: 0 }
    }
}

impl Strategy for DemoCrossStrategy {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tick(symbol: &str, price: i64) -> Tick {
        Tick {
            symbol: String::from(symbol),
            price,
            quantity: 1,
            timestamp: 1_717_000_000,
        }
    }

    #[test]
    fn strategy_generates_buy_order_when_price_is_below_threshold() {
        let mut strategy = ThresholdStrategy::new(String::from("BTCUSDT"), 99_000, 101_000, 1);

        let tick = create_tick("BTCUSDT", 98_000);

        let requests = strategy.on_tick(&tick);

        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].symbol, "BTCUSDT");
        assert_eq!(requests[0].side, Side::Buy);
        assert_eq!(requests[0].price, 98_000);
        assert_eq!(requests[0].quantity, 1);
    }

    #[test]
    fn strategy_generates_sell_order_when_price_is_above_threshold() {
        let mut strategy = ThresholdStrategy::new(String::from("BTCUSDT"), 99_000, 101_000, 1);

        let tick = create_tick("BTCUSDT", 102_000);

        let requests = strategy.on_tick(&tick);

        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].side, Side::Sell);
        assert_eq!(requests[0].price, 102_000);
    }

    #[test]
    fn strategy_generates_no_order_when_price_is_between_thresholds() {
        let mut strategy = ThresholdStrategy::new(String::from("BTCUSDT"), 99_000, 101_000, 1);

        let tick = create_tick("BTCUSDT", 100_000);

        let requests = strategy.on_tick(&tick);

        assert!(requests.is_empty());
    }

    #[test]
    fn strategy_ignores_ticks_for_other_symbols() {
        let mut strategy = ThresholdStrategy::new(String::from("BTCUSDT"), 99_000, 101_000, 1);

        let tick = create_tick("ETHUSDT", 98_000);

        let requests = strategy.on_tick(&tick);

        assert!(requests.is_empty());
    }

    #[test]
    fn demo_cross_strategy_generates_crossed_buy_then_sell_orders() {
        let mut strategy = DemoCrossStrategy::new();

        let tick1 = create_tick("BTCUSDT", 100_000);
        let tick2 = create_tick("BTCUSDT", 99_000);
        let tick3 = create_tick("BTCUSDT", 98_000);

        let requests1 = strategy.on_tick(&tick1);
        let requests2 = strategy.on_tick(&tick2);
        let requests3 = strategy.on_tick(&tick3);

        assert_eq!(requests1.len(), 1);
        assert_eq!(requests1[0].side, Side::Buy);
        assert_eq!(requests1[0].price, 100_000);
        assert_eq!(requests1[0].quantity, 2);

        assert_eq!(requests2.len(), 1);
        assert_eq!(requests2[0].side, Side::Sell);
        assert_eq!(requests2[0].price, 99_000);
        assert_eq!(requests2[0].quantity, 1);

        assert!(requests3.is_empty());
    }
}
