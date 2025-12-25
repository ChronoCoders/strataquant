pub mod buy_and_hold;
pub mod sma_crossover;
mod r#trait;

pub use buy_and_hold::BuyAndHold;
pub use r#trait::Strategy;
pub use sma_crossover::SMACrossover;
