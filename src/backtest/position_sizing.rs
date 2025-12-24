use serde::{Deserialize, Serialize};

/// Position sizing methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionSizingMethod {
    /// Fixed dollar amount per trade
    FixedDollar(f64),
    
    /// Fixed percentage of equity per trade
    FixedPercent(f64),
    
    /// Kelly Criterion (aggressive)
    Kelly { win_rate: f64, avg_win: f64, avg_loss: f64 },
    
    /// Half Kelly (more conservative)
    HalfKelly { win_rate: f64, avg_win: f64, avg_loss: f64 },
    
    /// Fixed Fractional (risk-based)
    FixedFractional { risk_per_trade: f64 },
}

impl PositionSizingMethod {
    /// Calculate position size based on current equity and method
    pub fn calculate_size(&self, equity: f64, risk_amount: Option<f64>) -> f64 {
        match self {
            PositionSizingMethod::FixedDollar(amount) => {
                if *amount > equity {
                    equity
                } else {
                    *amount
                }
            }
            
            PositionSizingMethod::FixedPercent(percent) => {
                equity * (percent / 100.0)
            }
            
            PositionSizingMethod::Kelly { win_rate, avg_win, avg_loss } => {
                let kelly = Self::kelly_criterion(*win_rate, *avg_win, *avg_loss);
                (equity * kelly).min(equity)
            }
            
            PositionSizingMethod::HalfKelly { win_rate, avg_win, avg_loss } => {
                let kelly = Self::kelly_criterion(*win_rate, *avg_win, *avg_loss);
                (equity * kelly * 0.5).min(equity)
            }
            
            PositionSizingMethod::FixedFractional { risk_per_trade } => {
                if let Some(risk) = risk_amount {
                    let max_loss = equity * (risk_per_trade / 100.0);
                    if risk > 0.0 {
                        max_loss / risk
                    } else {
                        equity * (risk_per_trade / 100.0)
                    }
                } else {
                    equity * (risk_per_trade / 100.0)
                }
            }
        }
    }
    
    /// Kelly Criterion formula: (W * R - L) / R
    /// W = win probability, R = avg_win/avg_loss, L = loss probability
    fn kelly_criterion(win_rate: f64, avg_win: f64, avg_loss: f64) -> f64 {
        if avg_loss == 0.0 {
            return 0.25; // Default conservative sizing if no losses
        }
        
        let win_prob = win_rate;
        let loss_prob = 1.0 - win_rate;
        let win_loss_ratio = avg_win / avg_loss;
        
        let kelly = (win_prob * win_loss_ratio - loss_prob) / win_loss_ratio;
        
        // Cap Kelly at 25% to prevent over-leveraging
        kelly.clamp(0.0, 0.25)
    }
}

impl Default for PositionSizingMethod {
    fn default() -> Self {
        // Default: 100% of equity (backwards compatible with v0.3.0)
        PositionSizingMethod::FixedPercent(100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_dollar() {
        let sizing = PositionSizingMethod::FixedDollar(10000.0);
        assert_eq!(sizing.calculate_size(100000.0, None), 10000.0);
        assert_eq!(sizing.calculate_size(5000.0, None), 5000.0); // Can't exceed equity
    }

    #[test]
    fn test_fixed_percent() {
        let sizing = PositionSizingMethod::FixedPercent(10.0);
        assert_eq!(sizing.calculate_size(100000.0, None), 10000.0);
        assert_eq!(sizing.calculate_size(50000.0, None), 5000.0);
    }

    #[test]
    fn test_kelly() {
        let sizing = PositionSizingMethod::Kelly {
            win_rate: 0.6,
            avg_win: 1000.0,
            avg_loss: 500.0,
        };
        let size = sizing.calculate_size(100000.0, None);
        assert!(size > 0.0 && size <= 100000.0);
    }
}
