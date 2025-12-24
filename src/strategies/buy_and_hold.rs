use crate::data::OHLCV;
use crate::strategies::Strategy;

/// Buy and hold strategy: enter at first bar, hold until last bar
pub struct BuyAndHold;

impl BuyAndHold {
    pub fn new() -> Self {
        Self
    }
}

impl Strategy for BuyAndHold {
    fn generate_signals(&self, data: &[OHLCV]) -> Vec<f64> {
        // Always fully invested
        vec![1.0; data.len()]
    }

    fn name(&self) -> &str {
        "Buy and Hold"
    }

    fn description(&self) -> String {
        "Buy at first bar, hold until last bar. Baseline benchmark strategy.".to_string()
    }
}

impl Default for BuyAndHold {
    fn default() -> Self {
        Self::new()
    }
}
