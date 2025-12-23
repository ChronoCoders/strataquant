pub fn calculate_max_drawdown(equity_curve: &[f64]) -> f64 {
    if equity_curve.is_empty() {
        return 0.0;
    }

    let mut max_equity = equity_curve[0];
    let mut max_dd = 0.0;

    for &equity in equity_curve {
        if equity > max_equity {
            max_equity = equity;
        }

        let drawdown = (equity - max_equity) / max_equity;
        if drawdown < max_dd {
            max_dd = drawdown;
        }
    }

    max_dd
}
