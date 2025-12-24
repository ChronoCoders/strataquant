/// Calculate Calmar ratio: annualized return / absolute max drawdown
///
/// Calmar ratio measures return per unit of drawdown risk.
/// Higher is better. Above 1.0 is good, above 3.0 is excellent.
///
/// Formula: (Total Return / Years) / |Max Drawdown|
pub fn calculate_calmar_ratio(total_return: f64, max_drawdown: f64, total_days: usize) -> f64 {
    if max_drawdown == 0.0 {
        return 0.0;
    }

    let years = total_days as f64 / 365.25;
    if years == 0.0 {
        return 0.0;
    }

    let annualized_return = total_return / years;
    let abs_max_drawdown = max_drawdown.abs();

    if abs_max_drawdown == 0.0 {
        return 0.0;
    }

    annualized_return / abs_max_drawdown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calmar_ratio() {
        // 100% return over 1 year with 50% drawdown
        let calmar = calculate_calmar_ratio(1.0, -0.5, 365);
        assert!((calmar - 2.0).abs() < 0.01);

        // 200% return over 2 years with 40% drawdown
        let calmar = calculate_calmar_ratio(2.0, -0.4, 730);
        assert!((calmar - 2.5).abs() < 0.01);
    }
}
