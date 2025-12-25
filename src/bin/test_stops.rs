use std::path::Path;
use strataquant::backtest::{BacktestEngine, ExecutionModel, StopLossMethod};
use strataquant::data::load_from_parquet;
use strataquant::strategies::SMACrossover;

fn main() {
    // Load data
    let data =
        load_from_parquet(Path::new("data/processed/btc_1d.parquet")).expect("Failed to load data");

    // Setup
    let capital = 100_000.0;
    let execution_model = ExecutionModel::new(10.0, 5.0);

    // Use SMA 20/50 for more trades
    let strategy = SMACrossover::new(20, 50);

    println!("=== SMA 20/50 - NO STOPS ===");
    let engine = BacktestEngine::new(data.clone(), capital, execution_model.clone());
    let result = engine.run(&strategy);
    println!("Return: {:.2}%", result.total_return * 100.0);
    println!("Max DD: {:.2}%", result.max_drawdown * 100.0);
    println!("Trades: {}\n", result.total_trades);

    println!("=== SMA 20/50 - 10% TRAILING STOP ===");
    let engine = BacktestEngine::new(data.clone(), capital, execution_model.clone())
        .with_stop_loss(StopLossMethod::Trailing(10.0));
    let result = engine.run(&strategy);
    println!("Return: {:.2}%", result.total_return * 100.0);
    println!("Max DD: {:.2}%", result.max_drawdown * 100.0);
    println!("Trades: {}\n", result.total_trades);

    println!("=== SMA 20/50 - 5% TRAILING STOP ===");
    let engine = BacktestEngine::new(data.clone(), capital, execution_model.clone())
        .with_stop_loss(StopLossMethod::Trailing(5.0));
    let result = engine.run(&strategy);
    println!("Return: {:.2}%", result.total_return * 100.0);
    println!("Max DD: {:.2}%", result.max_drawdown * 100.0);
    println!("Trades: {}\n", result.total_trades);

    println!("=== SMA 20/50 - 15% FIXED STOP ===");
    let engine = BacktestEngine::new(data, capital, execution_model)
        .with_stop_loss(StopLossMethod::FixedPercent(15.0));
    let result = engine.run(&strategy);
    println!("Return: {:.2}%", result.total_return * 100.0);
    println!("Max DD: {:.2}%", result.max_drawdown * 100.0);
    println!("Trades: {}", result.total_trades);
}
