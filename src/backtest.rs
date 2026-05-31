use crate::engine::Engine;
use crate::event::Event;
use crate::fill_simulator::SimpleFillSimulator;
use crate::model::Tick;
use crate::portfolio::Portfolio;
use crate::strategy::Strategy;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquityPoint {
    pub timestamp: u64,
    pub equity: i128,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BacktestResult {
    pub tick_count: usize,
    pub order_request_count: usize,
    pub trade_count: usize,
    pub order_update_count: usize,
    pub event_count: usize,
    pub simulated_fill_count: usize,
    pub initial_cash: i128,
    pub final_cash: i128,
    pub final_equity: i128,
    pub total_pnl: i128,
    pub fee_paid: i128,
    pub max_drawdown: i128,
    pub equity_curve: Vec<EquityPoint>,
    pub portfolio_trade_count: usize,
    pub final_positions: HashMap<String, i64>,
}

fn calculate_max_drawdown(initial_equity: i128, equity_curve: &[EquityPoint]) -> i128 {
    let mut peak = initial_equity;
    let mut max_drawdown = 0;

    for point in equity_curve {
        if point.equity > peak {
            peak = point.equity;
        }

        let drawdown = peak - point.equity;

        if drawdown > max_drawdown {
            max_drawdown = drawdown;
        }
    }

    max_drawdown
}

fn calculate_fee(price: i64, quantity: u64, fee_bps: u32) -> i128 {
    price as i128 * quantity as i128 * fee_bps as i128 / 10_000
}

pub struct BacktestEngine<S>
where
    S: Strategy,
{
    strategy: S,
    engine: Engine,
    portfolio: Portfolio,
    fee_bps: u32,
}

impl<S> BacktestEngine<S>
where
    S: Strategy,
{
    pub fn new(strategy: S) -> Self {
        Self::new_with_initial_cash_and_fee(strategy, 0, 0)
    }

    pub fn new_with_initial_cash(strategy: S, initial_cash: i128) -> Self {
        Self::new_with_initial_cash_and_fee(strategy, initial_cash, 0)
    }

    pub fn new_with_initial_cash_and_fee(strategy: S, initial_cash: i128, fee_bps: u32) -> Self {
        Self {
            strategy,
            engine: Engine::new(),
            portfolio: Portfolio::with_initial_cash(initial_cash),
            fee_bps,
        }
    }

    pub fn run(&mut self, ticks: &[Tick]) -> BacktestResult {
        let mut order_request_count = 0;
        let mut trade_count = 0;
        let mut order_update_count = 0;
        let mut event_count = 0;
        let mut simulated_fill_count = 0;
        let mut last_prices = HashMap::new();
        let mut equity_curve = Vec::new();

        let mut sorted_ticks: Vec<&Tick> = ticks.iter().collect();
        sorted_ticks.sort_by_key(|tick| tick.timestamp);

        for tick in sorted_ticks {
            last_prices.insert(tick.symbol.clone(), tick.price);

            let order_requests = self.strategy.on_tick(tick);
            order_request_count += order_requests.len();

            for request in order_requests {
                if let Some(fill) = SimpleFillSimulator::fill_order(&request, tick) {
                    simulated_fill_count += 1;

                    let fee = calculate_fee(fill.price, fill.quantity, self.fee_bps);

                    self.portfolio.apply_fill_with_fee(
                        &fill.symbol,
                        fill.side,
                        fill.price,
                        fill.quantity,
                        fee,
                    );
                }

                let events = self
                    .engine
                    .handle_event(Event::OrderRequest(request))
                    .expect("engine failed to handle order request during backtest");

                event_count += events.len();

                for event in events {
                    match event {
                        Event::Trade(_) => {
                            trade_count += 1;
                        }
                        Event::OrderUpdate(_) => {
                            order_update_count += 1;
                        }
                        _ => {}
                    }
                }
            }

            let equity = self.portfolio.equity(&last_prices);

            equity_curve.push(EquityPoint {
                timestamp: tick.timestamp,
                equity,
            });
        }

        let initial_cash = self.portfolio.initial_cash();
        let final_equity = self.portfolio.equity(&last_prices);
        let total_pnl = final_equity - initial_cash;

        BacktestResult {
            tick_count: ticks.len(),
            order_request_count,
            trade_count,
            order_update_count,
            event_count,
            simulated_fill_count,
            initial_cash,
            final_cash: self.portfolio.cash(),
            final_equity,
            total_pnl,
            fee_paid: self.portfolio.fee_paid(),
            max_drawdown: calculate_max_drawdown(initial_cash, &equity_curve),
            equity_curve,
            portfolio_trade_count: self.portfolio.trade_count(),
            final_positions: self.portfolio.position_quantities(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Tick;
    use crate::strategy::DemoCrossStrategy;

    #[test]
    fn backtest_engine_runs_ticks_through_strategy_and_engine() {
        let ticks = vec![
            Tick {
                symbol: "BTCUSDT".to_string(),
                price: 100_000,
                quantity: 1,
                timestamp: 1,
            },
            Tick {
                symbol: "BTCUSDT".to_string(),
                price: 99_000,
                quantity: 1,
                timestamp: 2,
            },
        ];

        let strategy = DemoCrossStrategy::new();
        let mut backtest_engine = BacktestEngine::new(strategy);

        let result = backtest_engine.run(&ticks);

        assert_eq!(result.tick_count, 2);
        assert_eq!(result.order_request_count, 2);
        assert_eq!(result.trade_count, 1);
        assert_eq!(result.order_update_count, 4);
        assert_eq!(result.event_count, 5);

        assert_eq!(result.simulated_fill_count, 2);
        assert_eq!(result.initial_cash, 0);
        assert_eq!(result.final_cash, -1_000);
        assert_eq!(result.final_equity, -1_000);
        assert_eq!(result.total_pnl, -1_000);
        assert_eq!(result.fee_paid, 0);
        assert_eq!(result.max_drawdown, 1_000);
        assert_eq!(result.portfolio_trade_count, 2);
        assert_eq!(result.final_positions.get("BTCUSDT"), Some(&0));

        assert_eq!(result.equity_curve.len(), 2);

        assert_eq!(
            result.equity_curve[0],
            EquityPoint {
                timestamp: 1,
                equity: 0,
            }
        );

        assert_eq!(
            result.equity_curve[1],
            EquityPoint {
                timestamp: 2,
                equity: -1_000,
            }
        );
    }

    #[test]
    fn backtest_engine_sorts_ticks_by_timestamp() {
        let ticks = vec![
            Tick {
                symbol: "BTCUSDT".to_string(),
                price: 99_000,
                quantity: 1,
                timestamp: 2,
            },
            Tick {
                symbol: "BTCUSDT".to_string(),
                price: 100_000,
                quantity: 1,
                timestamp: 1,
            },
        ];

        let strategy = DemoCrossStrategy::new();
        let mut backtest_engine = BacktestEngine::new(strategy);

        let result = backtest_engine.run(&ticks);

        assert_eq!(result.tick_count, 2);
        assert_eq!(result.order_request_count, 2);
        assert_eq!(result.trade_count, 1);
        assert_eq!(result.order_update_count, 4);
        assert_eq!(result.event_count, 5);

        assert_eq!(result.simulated_fill_count, 2);
        assert_eq!(result.initial_cash, 0);
        assert_eq!(result.final_cash, -1_000);
        assert_eq!(result.final_equity, -1_000);
        assert_eq!(result.total_pnl, -1_000);
        assert_eq!(result.fee_paid, 0);
        assert_eq!(result.max_drawdown, 1_000);
        assert_eq!(result.portfolio_trade_count, 2);
        assert_eq!(result.final_positions.get("BTCUSDT"), Some(&0));

        assert_eq!(result.equity_curve.len(), 2);

        assert_eq!(
            result.equity_curve[0],
            EquityPoint {
                timestamp: 1,
                equity: 0,
            }
        );

        assert_eq!(
            result.equity_curve[1],
            EquityPoint {
                timestamp: 2,
                equity: -1_000,
            }
        );
    }

    #[test]
    fn calculate_max_drawdown_returns_zero_for_empty_curve() {
        let equity_curve = vec![];

        assert_eq!(calculate_max_drawdown(100_000, &equity_curve), 0);
    }

    #[test]
    fn calculate_max_drawdown_returns_zero_when_equity_only_rises() {
        let equity_curve = vec![
            EquityPoint {
                timestamp: 1,
                equity: 0,
            },
            EquityPoint {
                timestamp: 2,
                equity: 500,
            },
            EquityPoint {
                timestamp: 3,
                equity: 1_000,
            },
        ];

        assert_eq!(calculate_max_drawdown(0, &equity_curve), 0);
    }

    #[test]
    fn calculate_max_drawdown_tracks_largest_drop_from_peak() {
        let equity_curve = vec![
            EquityPoint {
                timestamp: 1,
                equity: 0,
            },
            EquityPoint {
                timestamp: 2,
                equity: 1_000,
            },
            EquityPoint {
                timestamp: 3,
                equity: 400,
            },
            EquityPoint {
                timestamp: 4,
                equity: 800,
            },
            EquityPoint {
                timestamp: 5,
                equity: -200,
            },
        ];

        assert_eq!(calculate_max_drawdown(0, &equity_curve), 1_200);
    }

    #[test]
    fn backtest_engine_calculates_total_pnl_from_initial_cash() {
        let ticks = vec![
            Tick {
                symbol: "BTCUSDT".to_string(),
                price: 100_000,
                quantity: 1,
                timestamp: 1,
            },
            Tick {
                symbol: "BTCUSDT".to_string(),
                price: 99_000,
                quantity: 1,
                timestamp: 2,
            },
        ];

        let strategy = DemoCrossStrategy::new();
        let mut backtest_engine = BacktestEngine::new_with_initial_cash(strategy, 100_000);

        let result = backtest_engine.run(&ticks);

        assert_eq!(result.initial_cash, 100_000);
        assert_eq!(result.final_cash, 99_000);
        assert_eq!(result.final_equity, 99_000);
        assert_eq!(result.total_pnl, -1_000);
        assert_eq!(result.fee_paid, 0);
        assert_eq!(result.max_drawdown, 1_000);

        assert_eq!(result.simulated_fill_count, 2);
        assert_eq!(result.portfolio_trade_count, 2);
        assert_eq!(result.final_positions.get("BTCUSDT"), Some(&0));

        assert_eq!(result.equity_curve.len(), 2);

        assert_eq!(
            result.equity_curve[0],
            EquityPoint {
                timestamp: 1,
                equity: 100_000,
            }
        );

        assert_eq!(
            result.equity_curve[1],
            EquityPoint {
                timestamp: 2,
                equity: 99_000,
            }
        );
    }

    #[test]
    fn backtest_engine_applies_fee_to_simulated_fills() {
        let ticks = vec![
            Tick {
                symbol: "BTCUSDT".to_string(),
                price: 100_000,
                quantity: 1,
                timestamp: 1,
            },
            Tick {
                symbol: "BTCUSDT".to_string(),
                price: 99_000,
                quantity: 1,
                timestamp: 2,
            },
        ];

        let strategy = DemoCrossStrategy::new();

        // 10 bps = 0.10%
        let mut backtest_engine =
            BacktestEngine::new_with_initial_cash_and_fee(strategy, 100_000, 10);

        let result = backtest_engine.run(&ticks);

        assert_eq!(result.initial_cash, 100_000);

        // buy fee: 100000 * 10 / 10000 = 100
        // sell fee: 99000 * 10 / 10000 = 99
        assert_eq!(result.fee_paid, 199);

        assert_eq!(result.final_cash, 98_801);
        assert_eq!(result.final_equity, 98_801);
        assert_eq!(result.total_pnl, -1_199);
        assert_eq!(result.max_drawdown, 1_199);

        assert_eq!(result.simulated_fill_count, 2);
        assert_eq!(result.portfolio_trade_count, 2);
        assert_eq!(result.final_positions.get("BTCUSDT"), Some(&0));

        assert_eq!(result.equity_curve.len(), 2);

        assert_eq!(
            result.equity_curve[0],
            EquityPoint {
                timestamp: 1,
                equity: 99_900,
            }
        );

        assert_eq!(
            result.equity_curve[1],
            EquityPoint {
                timestamp: 2,
                equity: 98_801,
            }
        );
    }

    #[test]
    fn calculate_max_drawdown_uses_initial_equity_as_starting_peak() {
        let equity_curve = vec![
            EquityPoint {
                timestamp: 1,
                equity: 99_900,
            },
            EquityPoint {
                timestamp: 2,
                equity: 98_801,
            },
        ];

        assert_eq!(calculate_max_drawdown(100_000, &equity_curve), 1_199);
    }
}
