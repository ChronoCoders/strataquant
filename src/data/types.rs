use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHLCV {
    pub timestamp: i64, // Unix timestamp in milliseconds
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl OHLCV {
    pub fn is_valid(&self) -> bool {
        // Verify OHLC constraints
        self.high >= self.low
            && self.high >= self.open
            && self.high >= self.close
            && self.low <= self.open
            && self.low <= self.close
            && self.open > 0.0
            && self.high > 0.0
            && self.low > 0.0
            && self.close > 0.0
            && self.volume >= 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ohlcv() {
        let bar = OHLCV {
            timestamp: 1609459200000,
            open: 100.0,
            high: 110.0,
            low: 95.0,
            close: 105.0,
            volume: 1000.0,
        };
        assert!(bar.is_valid());
    }

    #[test]
    fn test_invalid_ohlcv_high_low() {
        let bar = OHLCV {
            timestamp: 1609459200000,
            open: 100.0,
            high: 90.0, // High < low is invalid
            low: 95.0,
            close: 105.0,
            volume: 1000.0,
        };
        assert!(!bar.is_valid());
    }
}
