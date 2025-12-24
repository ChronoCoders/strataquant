/// Calculate Sortino ratio: return / downside deviation
/// 
/// Similar to Sharpe ratio but only penalizes downside volatility.
/// Higher is better. Above 1.0 is good, above 2.0 is excellent.
/// 
/// Sortino is more useful than Sharpe for asymmetric return distributions
/// (common in trading where upside is unlimited but downside is limited by stops).
pub fn calculate_sortino_ratio(returns: &[f64], periods_per_year: f64) -> f64 {
    if returns.is_empty() {
        return 0.0;
    }

    let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;

    // Only consider negative returns for downside deviation
    let downside_returns: Vec<f64> = returns
        .iter()
        .filter(|&&r| r < 0.0)
        .copied()
        .collect();

    if downside_returns.is_empty() {
        // No negative returns = infinite Sortino (cap at 999.0)
        return 999.0;
    }

    let downside_variance = downside_returns
        .iter()
        .map(|&r| r.powi(2))
        .sum::<f64>()
        / returns.len() as f64; // Note: divide by total returns, not just downside

    let downside_deviation = downside_variance.sqrt();

    if downside_deviation == 0.0 {
        return 0.0;
    }

    (mean_return / downside_deviation) * periods_per_year.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sortino_ratio() {
        // Symmetric returns: should be similar to Sharpe
        let returns = vec![0.01, -0.01, 0.02, -0.02, 0.01];
        let sortino = calculate_sortino_ratio(&returns, 252.0);
        assert!(sortino.abs() < 5.0);

        // All positive returns: should be very high
        let returns = vec![0.01, 0.02, 0.01, 0.03];
        let sortino = calculate_sortino_ratio(&returns, 252.0);
        assert!(sortino > 100.0);

        // Empty returns
        let returns: Vec<f64> = vec![];
        let sortino = calculate_sortino_ratio(&returns, 252.0);
        assert_eq!(sortino, 0.0);
    }
}
