use serde::{Deserialize, Serialize};

/// Stop loss methods
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum StopLossMethod {
    /// No stop loss
    #[default]
    None,
    
    /// Fixed percentage from entry
    FixedPercent(f64),
    
    /// Trailing stop (percentage from highest price)
    Trailing(f64),
    
    /// ATR-based stop (multiplier * ATR)
    ATR { multiplier: f64, period: usize },
    
    /// Time-based exit (max bars in position)
    TimeLimit(usize),
}

impl StopLossMethod {
    /// Check if stop is hit
    /// Returns true if position should be closed
    pub fn is_hit(
        &self,
        entry_price: f64,
        current_price: f64,
        highest_price: f64,
        bars_held: usize,
        atr: Option<f64>,
    ) -> bool {
        match self {
            StopLossMethod::None => false,
            
            StopLossMethod::FixedPercent(percent) => {
                let stop_price = entry_price * (1.0 - percent / 100.0);
                current_price <= stop_price
            }
            
            StopLossMethod::Trailing(percent) => {
                let stop_price = highest_price * (1.0 - percent / 100.0);
                current_price <= stop_price
            }
            
            StopLossMethod::ATR { multiplier, .. } => {
                if let Some(atr_value) = atr {
                    let stop_distance = atr_value * multiplier;
                    let stop_price = entry_price - stop_distance;
                    current_price <= stop_price
                } else {
                    false
                }
            }
            
            StopLossMethod::TimeLimit(max_bars) => {
                bars_held >= *max_bars
            }
        }
    }
    
    /// Get the actual stop price (for display/logging)
    pub fn get_stop_price(
        &self,
        entry_price: f64,
        highest_price: f64,
        atr: Option<f64>,
    ) -> Option<f64> {
        match self {
            StopLossMethod::None => None,
            
            StopLossMethod::FixedPercent(percent) => {
                Some(entry_price * (1.0 - percent / 100.0))
            }
            
            StopLossMethod::Trailing(percent) => {
                Some(highest_price * (1.0 - percent / 100.0))
            }
            
            StopLossMethod::ATR { multiplier, .. } => {
                atr.map(|atr_value| entry_price - (atr_value * multiplier))
            }
            
            StopLossMethod::TimeLimit(_) => None,
        }
    }
}

/// Calculate Average True Range (ATR)
pub fn calculate_atr(data: &[(f64, f64, f64)], period: usize) -> Vec<f64> {
    // data: Vec<(high, low, close)>
    let mut atr = Vec::with_capacity(data.len());
    let mut true_ranges = Vec::with_capacity(data.len());
    
    for i in 0..data.len() {
        let (high, low, _close) = data[i];
        
        let tr = if i == 0 {
            high - low
        } else {
            let prev_close = data[i - 1].2;
            let tr1 = high - low;
            let tr2 = (high - prev_close).abs();
            let tr3 = (low - prev_close).abs();
            tr1.max(tr2).max(tr3)
        };
        
        true_ranges.push(tr);
        
        if i < period - 1 {
            atr.push(f64::NAN);
        } else if i == period - 1 {
            let avg_tr: f64 = true_ranges[..period].iter().sum::<f64>() / period as f64;
            atr.push(avg_tr);
        } else {
            let prev_atr = atr[i - 1];
            let smoothed_atr = (prev_atr * (period as f64 - 1.0) + tr) / period as f64;
            atr.push(smoothed_atr);
        }
    }
    
    atr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_stop() {
        let stop = StopLossMethod::FixedPercent(10.0);
        
        // Entry at 100, 10% stop at 90
        assert!(!stop.is_hit(100.0, 91.0, 100.0, 0, None));
        assert!(stop.is_hit(100.0, 89.0, 100.0, 0, None));
    }

    #[test]
    fn test_trailing_stop() {
        let stop = StopLossMethod::Trailing(10.0);
        
        // Entry 100, highest 120, 10% trailing stop at 108
        assert!(!stop.is_hit(100.0, 109.0, 120.0, 0, None));
        assert!(stop.is_hit(100.0, 107.0, 120.0, 0, None));
    }

    #[test]
    fn test_time_limit() {
        let stop = StopLossMethod::TimeLimit(10);
        
        assert!(!stop.is_hit(100.0, 110.0, 110.0, 9, None));
        assert!(stop.is_hit(100.0, 110.0, 110.0, 10, None));
    }

    #[test]
    fn test_atr_calculation() {
        let data = vec![
            (10.0, 9.0, 9.5),
            (10.5, 9.5, 10.0),
            (11.0, 10.0, 10.5),
        ];
        
        let atr = calculate_atr(&data, 2);
        assert_eq!(atr.len(), 3);
        assert!(atr[0].is_nan());
        assert!(atr[1] > 0.0);
    }
}
