pub mod engine;
pub mod position_sizing;
pub mod result;
pub mod risk;
pub mod stops;
pub mod trade;
pub mod types;

pub use engine::BacktestEngine;
pub use position_sizing::PositionSizingMethod;
pub use result::BacktestResult;
pub use risk::{RiskLimits, RiskMetrics};
pub use stops::{calculate_atr, StopLossMethod};
pub use trade::{Trade, TradeStats};
pub use types::{ExecutionModel, Portfolio};
