mod metrics;
mod result;

#[cfg(test)]
mod tests;

use crate::engine::Engine;
use crate::event::Event;
use crate::fill_simulator::SimpleFillSimulator;
use crate::model::{OrderRequest, Tick};
use crate::portfolio::Portfolio;
use crate::strategy::Strategy;
use metrics::{calculate_fee, calculate_max_drawdown};
use std::collections::HashMap;

pub use result::{BacktestResult, EquityPoint};

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

        for tick in self.sorted_ticks(ticks) {
            last_prices.insert(tick.symbol.clone(), tick.price);

            let order_requests = self.strategy.on_tick(tick);
            order_request_count += order_requests.len();

            for request in order_requests {
                if self.apply_simulated_fill(&request, tick) {
                    simulated_fill_count += 1;
                }

                let events = self.collect_engine_events(request);
                event_count += events.len();

                let (new_trade_count, new_order_update_count) = Self::count_events(&events);
                trade_count += new_trade_count;
                order_update_count += new_order_update_count;
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

    fn sorted_ticks<'a>(&self, ticks: &'a [Tick]) -> Vec<&'a Tick> {
        let mut sorted_ticks: Vec<&Tick> = ticks.iter().collect();
        sorted_ticks.sort_by_key(|tick| tick.timestamp);
        sorted_ticks
    }

    fn apply_simulated_fill(&mut self, request: &OrderRequest, tick: &Tick) -> bool {
        let Some(fill) = SimpleFillSimulator::fill_order(request, tick) else {
            return false;
        };

        let fee = calculate_fee(fill.price, fill.quantity, self.fee_bps);

        self.portfolio
            .apply_fill_with_fee(&fill.symbol, fill.side, fill.price, fill.quantity, fee);

        true
    }

    fn collect_engine_events(&mut self, request: OrderRequest) -> Vec<Event> {
        self.engine
            .handle_event(Event::OrderRequest(request))
            .expect("engine failed to handle order request during backtest")
    }

    fn count_events(events: &[Event]) -> (usize, usize) {
        let mut trade_count = 0;
        let mut order_update_count = 0;

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

        (trade_count, order_update_count)
    }
}
