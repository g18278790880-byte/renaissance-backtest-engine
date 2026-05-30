use crate::model::{OrderRequest, Side, Tick};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulatedFill {
    pub symbol: String,
    pub side: Side,
    pub price: i64,
    pub quantity: u64,
    pub timestamp: u64,
}

pub struct SimpleFillSimulator;

impl SimpleFillSimulator {
    pub fn fill_order(request: &OrderRequest, tick: &Tick) -> Option<SimulatedFill> {
        if request.symbol != tick.symbol {
            return None;
        }

        let should_fill = match request.side {
            Side::Buy => request.price >= tick.price,
            Side::Sell => request.price <= tick.price,
        };

        if !should_fill {
            return None;
        }

        Some(SimulatedFill {
            symbol: request.symbol.clone(),
            side: request.side,
            price: tick.price,
            quantity: request.quantity.min(tick.quantity),
            timestamp: tick.timestamp,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tick(price: i64) -> Tick {
        Tick {
            symbol: "BTCUSDT".to_string(),
            price,
            quantity: 10,
            timestamp: 1,
        }
    }

    fn order(side: Side, price: i64, quantity: u64) -> OrderRequest {
        OrderRequest {
            symbol: "BTCUSDT".to_string(),
            side,
            price,
            quantity,
        }
    }

    #[test]
    fn buy_limit_order_fills_when_limit_price_is_above_or_equal_market_price() {
        let request = order(Side::Buy, 100_000, 2);
        let fill = SimpleFillSimulator::fill_order(&request, &tick(99_000)).unwrap();

        assert_eq!(fill.symbol, "BTCUSDT");
        assert_eq!(fill.side, Side::Buy);
        assert_eq!(fill.price, 99_000);
        assert_eq!(fill.quantity, 2);
        assert_eq!(fill.timestamp, 1);
    }

    #[test]
    fn buy_limit_order_does_not_fill_when_limit_price_is_below_market_price() {
        let request = order(Side::Buy, 98_000, 2);
        let fill = SimpleFillSimulator::fill_order(&request, &tick(99_000));

        assert_eq!(fill, None);
    }

    #[test]
    fn sell_limit_order_fills_when_limit_price_is_below_or_equal_market_price() {
        let request = order(Side::Sell, 98_000, 2);
        let fill = SimpleFillSimulator::fill_order(&request, &tick(99_000)).unwrap();

        assert_eq!(fill.symbol, "BTCUSDT");
        assert_eq!(fill.side, Side::Sell);
        assert_eq!(fill.price, 99_000);
        assert_eq!(fill.quantity, 2);
        assert_eq!(fill.timestamp, 1);
    }

    #[test]
    fn sell_limit_order_does_not_fill_when_limit_price_is_above_market_price() {
        let request = order(Side::Sell, 100_000, 2);
        let fill = SimpleFillSimulator::fill_order(&request, &tick(99_000));

        assert_eq!(fill, None);
    }

    #[test]
    fn order_does_not_fill_when_symbol_is_different() {
        let request = OrderRequest {
            symbol: "ETHUSDT".to_string(),
            side: Side::Buy,
            price: 100_000,
            quantity: 2,
        };

        let fill = SimpleFillSimulator::fill_order(&request, &tick(99_000));

        assert_eq!(fill, None);
    }

    #[test]
    fn fill_quantity_is_limited_by_tick_quantity() {
        let request = order(Side::Buy, 100_000, 20);
        let fill = SimpleFillSimulator::fill_order(&request, &tick(99_000)).unwrap();

        assert_eq!(fill.quantity, 10);
    }
}
