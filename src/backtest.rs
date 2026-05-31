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
    pub final_cash: i128,
    pub final_equity: i128,
    pub max_drawdown: i128,
    pub equity_curve: Vec<EquityPoint>,
    pub portfolio_trade_count: usize,
    pub final_positions: HashMap<String, i64>,
}

fn calculate_max_drawdown(equity_curve: &[EquityPoint]) -> i128 {
    let mut peak: Option<i128> = None;
    let mut max_drawdown = 0;

    for point in equity_curve {
        let current_peak = match peak {
            Some(existing_peak) if existing_peak >= point.equity => existing_peak,
            _ => point.equity,
        };

        peak = Some(current_peak);

        let drawdown = current_peak - point.equity;

        if drawdown > max_drawdown {
            max_drawdown = drawdown;
        }
    }

    max_drawdown
}

pub struct BacktestEngine<S>
where
    S: Strategy,
{
    strategy: S,
    engine: Engine,
    portfolio: Portfolio,
}

impl<S> BacktestEngine<S>
where
    S: Strategy,
{
    pub fn new(strategy: S) -> Self {
        Self {
            strategy,
            engine: Engine::new(),
            portfolio: Portfolio::new(),
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

                    self.portfolio
                        .apply_fill(&fill.symbol, fill.side, fill.price, fill.quantity);
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

        BacktestResult {
            tick_count: ticks.len(),
            order_request_count,
            trade_count,
            order_update_count,
            event_count,
            simulated_fill_count,
            final_cash: self.portfolio.cash(),
            final_equity: self.portfolio.equity(&last_prices),
            max_drawdown: calculate_max_drawdown(&equity_curve),
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
        assert_eq!(result.final_cash, -1_000);
        assert_eq!(result.final_equity, -1_000);
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
        assert_eq!(result.final_cash, -1_000);
        assert_eq!(result.final_equity, -1_000);
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

        assert_eq!(calculate_max_drawdown(&equity_curve), 0);
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

        assert_eq!(calculate_max_drawdown(&equity_curve), 0);
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

        assert_eq!(calculate_max_drawdown(&equity_curve), 1_200);
    }
}
