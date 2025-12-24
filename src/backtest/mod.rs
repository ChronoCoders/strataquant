pub mod engine;
pub mod result;
pub mod trade;
pub mod types;

pub use engine::BacktestEngine;
pub use result::BacktestResult;
pub use trade::{Trade, TradeStats};
pub use types::{ExecutionModel, Portfolio};
