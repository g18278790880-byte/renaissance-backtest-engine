use super::*;
use crate::model::Tick;
use crate::strategy::DemoCrossStrategy;

fn demo_ticks() -> Vec<Tick> {
    vec![
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
    ]
}

fn reversed_demo_ticks() -> Vec<Tick> {
    vec![
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
    ]
}

#[test]
fn backtest_engine_runs_ticks_through_strategy_and_engine() {
    let ticks = demo_ticks();

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
    let ticks = reversed_demo_ticks();

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
    let ticks = demo_ticks();

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
    let ticks = demo_ticks();

    let strategy = DemoCrossStrategy::new();

    // 10 bps = 0.10%
    let mut backtest_engine = BacktestEngine::new_with_initial_cash_and_fee(strategy, 100_000, 10);

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

#[test]
fn backtest_result_can_be_converted_into_report() {
    let ticks = demo_ticks();

    let strategy = DemoCrossStrategy::new();
    let mut backtest_engine = BacktestEngine::new_with_initial_cash_and_fee(strategy, 100_000, 10);

    let result = backtest_engine.run(&ticks);
    let report = BacktestReport::from(result);

    assert_eq!(report.summary.tick_count, 2);
    assert_eq!(report.summary.order_request_count, 2);
    assert_eq!(report.summary.engine_trade_count, 1);
    assert_eq!(report.summary.engine_order_update_count, 4);
    assert_eq!(report.summary.engine_event_count, 5);
    assert_eq!(report.summary.simulated_fill_count, 2);
    assert_eq!(report.summary.portfolio_trade_count, 2);

    assert_eq!(report.metrics.initial_cash, 100_000);
    assert_eq!(report.metrics.final_cash, 98_801);
    assert_eq!(report.metrics.final_equity, 98_801);
    assert_eq!(report.metrics.total_pnl, -1_199);
    assert_eq!(report.metrics.fee_paid, 199);
    assert_eq!(report.metrics.max_drawdown, 1_199);

    assert_eq!(report.positions.len(), 1);
    assert_eq!(report.positions[0].symbol, "BTCUSDT");
    assert_eq!(report.positions[0].quantity, 0);

    assert_eq!(report.equity_curve.len(), 2);
    assert_eq!(report.equity_curve[0].timestamp, 1);
    assert_eq!(report.equity_curve[0].equity, 99_900);
    assert_eq!(report.equity_curve[1].timestamp, 2);
    assert_eq!(report.equity_curve[1].equity, 98_801);
}

#[test]
fn backtest_report_sorts_positions_by_symbol() {
    let mut final_positions = std::collections::HashMap::new();
    final_positions.insert("ETHUSDT".to_string(), 2);
    final_positions.insert("BTCUSDT".to_string(), 1);

    let result = BacktestResult {
        tick_count: 0,
        order_request_count: 0,
        trade_count: 0,
        order_update_count: 0,
        event_count: 0,
        simulated_fill_count: 0,
        initial_cash: 0,
        final_cash: 0,
        final_equity: 0,
        total_pnl: 0,
        fee_paid: 0,
        max_drawdown: 0,
        equity_curve: Vec::new(),
        portfolio_trade_count: 0,
        final_positions,
    };

    let report = BacktestReport::from(result);

    assert_eq!(report.positions[0].symbol, "BTCUSDT");
    assert_eq!(report.positions[0].quantity, 1);
    assert_eq!(report.positions[1].symbol, "ETHUSDT");
    assert_eq!(report.positions[1].quantity, 2);
}
