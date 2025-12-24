use crate::data::OHLCV;

/// Core trait that all trading strategies must implement
pub trait Strategy: Send + Sync {
    /// Generate trading signals for the given market data
    /// Returns a vector of positions where:
    /// - 1.0 = fully long (100% BTC)
    /// - 0.0 = fully flat (100% cash)
    fn generate_signals(&self, data: &[OHLCV]) -> Vec<f64>;

    /// Get the name of the strategy for display/logging
    fn name(&self) -> &str;

    /// Optional: Get description of the strategy
    fn description(&self) -> String {
        format!("{} strategy", self.name())
    }
}
