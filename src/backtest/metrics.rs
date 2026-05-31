use super::EquityPoint;

pub(crate) fn calculate_max_drawdown(initial_equity: i128, equity_curve: &[EquityPoint]) -> i128 {
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

pub(crate) fn calculate_fee(price: i64, quantity: u64, fee_bps: u32) -> i128 {
    price as i128 * quantity as i128 * fee_bps as i128 / 10_000
}
