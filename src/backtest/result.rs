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
