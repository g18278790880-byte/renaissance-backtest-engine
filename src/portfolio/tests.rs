use super::*;
use std::collections::HashMap;

fn last_prices(symbol: &str, price: i64) -> HashMap<String, i64> {
    let mut prices = HashMap::new();
    prices.insert(symbol.to_string(), price);
    prices
}

mod cash_and_position_tests {
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
    fn portfolio_can_start_with_initial_cash() {
        let portfolio = Portfolio::with_initial_cash(100_000);

        assert_eq!(portfolio.initial_cash(), 100_000);
        assert_eq!(portfolio.cash(), 100_000);
        assert_eq!(portfolio.trade_count(), 0);
    }
}

mod equity_tests {
    use super::*;

    #[test]
    fn portfolio_equity_marks_position_to_last_price() {
        let mut portfolio = Portfolio::new();

        portfolio.apply_fill("BTCUSDT", Side::Buy, 100_000, 1);

        assert_eq!(portfolio.cash(), -100_000);
        assert_eq!(portfolio.position_quantity("BTCUSDT"), 1);
        assert_eq!(portfolio.equity(&last_prices("BTCUSDT", 105_000)), 5_000);
    }

    #[test]
    fn portfolio_equity_marks_short_position_to_last_price() {
        let mut portfolio = Portfolio::new();

        portfolio.apply_fill("BTCUSDT", Side::Sell, 100_000, 1);

        assert_eq!(portfolio.cash(), 100_000);
        assert_eq!(portfolio.position_quantity("BTCUSDT"), -1);
        assert_eq!(portfolio.equity(&last_prices("BTCUSDT", 95_000)), 5_000);
    }
}

mod fee_tests {
    use super::*;

    #[test]
    fn portfolio_apply_buy_fill_with_fee_decreases_cash_by_notional_plus_fee() {
        let mut portfolio = Portfolio::with_initial_cash(100_000);

        portfolio.apply_fill_with_fee("BTCUSDT", Side::Buy, 100_000, 1, 100);

        assert_eq!(portfolio.position_quantity("BTCUSDT"), 1);
        assert_eq!(portfolio.cash(), -100);
        assert_eq!(portfolio.fee_paid(), 100);
        assert_eq!(portfolio.trade_count(), 1);
    }

    #[test]
    fn portfolio_apply_sell_fill_with_fee_increases_cash_by_notional_minus_fee() {
        let mut portfolio = Portfolio::with_initial_cash(0);

        portfolio.apply_fill_with_fee("BTCUSDT", Side::Sell, 100_000, 1, 100);

        assert_eq!(portfolio.position_quantity("BTCUSDT"), -1);
        assert_eq!(portfolio.cash(), 99_900);
        assert_eq!(portfolio.fee_paid(), 100);
        assert_eq!(portfolio.trade_count(), 1);
    }
}
