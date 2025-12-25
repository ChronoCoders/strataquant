use crate::data::OHLCV;
use crate::strategies::Strategy;

/// Simple Moving Average (SMA) Crossover Strategy
/// Go long when fast MA crosses above slow MA
/// Go flat when fast MA crosses below slow MA
pub struct SMACrossover {
    fast_period: usize,
    slow_period: usize,
}

impl SMACrossover {
    pub fn new(fast_period: usize, slow_period: usize) -> Self {
        assert!(
            fast_period < slow_period,
            "Fast period must be less than slow period"
        );
        assert!(fast_period > 0, "Fast period must be greater than 0");
        assert!(slow_period > 0, "Slow period must be greater than 0");

        Self {
            fast_period,
            slow_period,
        }
    }

    fn calculate_sma(&self, prices: &[f64], period: usize) -> Vec<f64> {
        let mut sma = Vec::with_capacity(prices.len());

        if period == 0 || prices.is_empty() {
            return vec![f64::NAN; prices.len()];
        }

        for i in 0..prices.len() {
            if i + 1 < period {
                sma.push(f64::NAN);
            } else {
                let start = i + 1 - period;
                let sum: f64 = prices[start..=i].iter().sum();
                sma.push(sum / period as f64);
            }
        }

        sma
    }
}

impl Strategy for SMACrossover {
    fn generate_signals(&self, data: &[OHLCV]) -> Vec<f64> {
        let closes: Vec<f64> = data.iter().map(|d| d.close).collect();

        let fast_sma = self.calculate_sma(&closes, self.fast_period);
        let slow_sma = self.calculate_sma(&closes, self.slow_period);

        let mut signals = Vec::with_capacity(data.len());
        let mut current_position = 0.0;

        for i in 0..data.len() {
            if fast_sma[i].is_nan() || slow_sma[i].is_nan() {
                signals.push(0.0);
                continue;
            }

            if i > 0 {
                let prev_fast = fast_sma[i - 1];
                let prev_slow = slow_sma[i - 1];

                // Golden cross: fast crosses above slow
                if !prev_fast.is_nan() && !prev_slow.is_nan() {
                    if prev_fast <= prev_slow && fast_sma[i] > slow_sma[i] {
                        current_position = 1.0;
                    }
                    // Death cross: fast crosses below slow
                    else if prev_fast >= prev_slow && fast_sma[i] < slow_sma[i] {
                        current_position = 0.0;
                    }
                }
            } else {
                // First valid signal
                current_position = if fast_sma[i] > slow_sma[i] { 1.0 } else { 0.0 };
            }

            signals.push(current_position);
        }

        signals
    }

    fn name(&self) -> &str {
        "SMA Crossover"
    }

    fn description(&self) -> String {
        format!(
            "SMA {}/{} crossover - Long when fast > slow, flat otherwise",
            self.fast_period, self.slow_period
        )
    }
}
