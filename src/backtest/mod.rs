pub mod types;
pub mod result;
pub mod engine;
pub mod trade;
pub mod position_sizing;
pub mod stops;
pub mod risk;

pub use types::{ExecutionModel, Portfolio};
pub use result::BacktestResult;
pub use engine::BacktestEngine;
pub use trade::{Trade, TradeStats};
pub use position_sizing::PositionSizingMethod;
pub use stops::{StopLossMethod, calculate_atr};
pub use risk::{RiskLimits, RiskMetrics};
