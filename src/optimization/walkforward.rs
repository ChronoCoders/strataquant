use crate::backtest::{BacktestEngine, ExecutionModel};
use crate::data::OHLCV;
use crate::optimization::ParameterSweep;
use crate::strategies::SMACrossover;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkForwardResult {
    pub train_size: usize,
    pub test_size: usize,
    pub best_fast_period: usize,
    pub best_slow_period: usize,
    pub in_sample_return: f64,
    pub in_sample_sharpe: f64,
    pub out_of_sample_return: f64,
    pub out_of_sample_sharpe: f64,
    pub degradation_return: f64,
    pub degradation_sharpe: f64,
}

pub struct WalkForward {
    data: Vec<OHLCV>,
    initial_capital: f64,
    execution_model: ExecutionModel,
}

impl WalkForward {
    pub fn new(data: Vec<OHLCV>, initial_capital: f64, execution_model: ExecutionModel) -> Self {
        Self {
            data,
            initial_capital,
            execution_model,
        }
    }

    pub fn run(&self, train_ratio: f64) -> WalkForwardResult {
        assert!(train_ratio > 0.0 && train_ratio < 1.0, "Train ratio must be between 0 and 1");

        let split_point = (self.data.len() as f64 * train_ratio) as usize;
        let train_data = &self.data[..split_point];
        let test_data = &self.data[split_point..];

        println!("Walk-Forward Validation");
        println!("======================");
        println!("Total bars: {}", self.data.len());
        println!("Train bars: {} ({:.1}%)", train_data.len(), train_ratio * 100.0);
        println!("Test bars:  {} ({:.1}%)\n", test_data.len(), (1.0 - train_ratio) * 100.0);

        println!("Phase 1: Optimization on training set...");
        let sweep = ParameterSweep::new(
            train_data.to_vec(),
            self.initial_capital,
            self.execution_model.clone(),
        );

        let results = sweep.sweep_sma_periods((20, 100), (50, 200), 10);

        let best = ParameterSweep::find_best_sharpe(&results)
            .expect("No optimization results");

        println!(
            "Best in-sample parameters: {}/{} (Sharpe: {:.2}, Return: {:.2}%)\n",
            best.fast_period,
            best.slow_period,
            best.sharpe_ratio,
            best.total_return * 100.0
        );

        println!("Phase 2: Testing on out-of-sample data...");
        let strategy = SMACrossover::new(best.fast_period, best.slow_period);
        let test_engine = BacktestEngine::new(
            test_data.to_vec(),
            self.initial_capital,
            self.execution_model.clone(),
        );
        let test_result = test_engine.run(&strategy);

        println!(
            "Out-of-sample: Sharpe: {:.2}, Return: {:.2}%\n",
            test_result.sharpe_ratio,
            test_result.total_return * 100.0
        );

        let degradation_return =
            ((best.total_return - test_result.total_return) / best.total_return.abs()) * 100.0;
        let degradation_sharpe =
            ((best.sharpe_ratio - test_result.sharpe_ratio) / best.sharpe_ratio.abs()) * 100.0;

        WalkForwardResult {
            train_size: train_data.len(),
            test_size: test_data.len(),
            best_fast_period: best.fast_period,
            best_slow_period: best.slow_period,
            in_sample_return: best.total_return,
            in_sample_sharpe: best.sharpe_ratio,
            out_of_sample_return: test_result.total_return,
            out_of_sample_sharpe: test_result.sharpe_ratio,
            degradation_return,
            degradation_sharpe,
        }
    }

    pub fn save_result(result: &WalkForwardResult, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(result)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
