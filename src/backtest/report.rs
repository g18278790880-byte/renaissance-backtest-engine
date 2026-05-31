use super::{BacktestResult, EquityPoint};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BacktestReport {
    pub summary: BacktestSummary,
    pub metrics: BacktestMetrics,
    pub positions: Vec<BacktestPosition>,
    pub equity_curve: Vec<EquityPoint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BacktestSummary {
    pub tick_count: usize,
    pub order_request_count: usize,
    pub engine_trade_count: usize,
    pub engine_order_update_count: usize,
    pub engine_event_count: usize,
    pub simulated_fill_count: usize,
    pub portfolio_trade_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BacktestMetrics {
    pub initial_cash: i128,
    pub final_cash: i128,
    pub final_equity: i128,
    pub total_pnl: i128,
    pub fee_paid: i128,
    pub max_drawdown: i128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BacktestPosition {
    pub symbol: String,
    pub quantity: i64,
}

impl From<BacktestResult> for BacktestReport {
    fn from(result: BacktestResult) -> Self {
        let mut positions: Vec<BacktestPosition> = result
            .final_positions
            .into_iter()
            .map(|(symbol, quantity)| BacktestPosition { symbol, quantity })
            .collect();

        positions.sort_by(|left, right| left.symbol.cmp(&right.symbol));

        Self {
            summary: BacktestSummary {
                tick_count: result.tick_count,
                order_request_count: result.order_request_count,
                engine_trade_count: result.trade_count,
                engine_order_update_count: result.order_update_count,
                engine_event_count: result.event_count,
                simulated_fill_count: result.simulated_fill_count,
                portfolio_trade_count: result.portfolio_trade_count,
            },
            metrics: BacktestMetrics {
                initial_cash: result.initial_cash,
                final_cash: result.final_cash,
                final_equity: result.final_equity,
                total_pnl: result.total_pnl,
                fee_paid: result.fee_paid,
                max_drawdown: result.max_drawdown,
            },
            positions,
            equity_curve: result.equity_curve,
        }
    }
}
