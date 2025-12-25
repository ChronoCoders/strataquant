use crate::backtest::{BacktestEngine, ExecutionModel, StopLossMethod};
use crate::data::OHLCV;
use crate::strategies::SMACrossover;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub fast_period: usize,
    pub slow_period: usize,
    pub total_return: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown: f64,
    pub total_trades: u32,
}

pub struct ParameterSweep {
    data: Vec<OHLCV>,
    initial_capital: f64,
    execution_model: ExecutionModel,
}

impl ParameterSweep {
    pub fn new(data: Vec<OHLCV>, initial_capital: f64, execution_model: ExecutionModel) -> Self {
        Self {
            data,
            initial_capital,
            execution_model,
        }
    }

    pub fn sweep_sma_periods(
        &self,
        fast_range: (usize, usize),
        slow_range: (usize, usize),
        step: usize,
    ) -> Vec<OptimizationResult> {
        let fast_periods: Vec<usize> = (fast_range.0..=fast_range.1).step_by(step).collect();
        let slow_periods: Vec<usize> = (slow_range.0..=slow_range.1).step_by(step).collect();

        let mut parameter_combinations = Vec::new();
        for fast in &fast_periods {
            for slow in &slow_periods {
                if fast < slow {
                    parameter_combinations.push((*fast, *slow));
                }
            }
        }

        println!(
            "Testing {} parameter combinations...",
            parameter_combinations.len()
        );

        let results: Vec<OptimizationResult> = parameter_combinations
            .par_iter()
            .map(|(fast, slow)| {
                let strategy = SMACrossover::new(*fast, *slow);
                let engine = BacktestEngine::new(
                    self.data.clone(),
                    self.initial_capital,
                    self.execution_model.clone(),
                );
                let backtest_result = engine.run(&strategy);

                OptimizationResult {
                    fast_period: *fast,
                    slow_period: *slow,
                    total_return: backtest_result.total_return,
                    sharpe_ratio: backtest_result.sharpe_ratio,
                    max_drawdown: backtest_result.max_drawdown,
                    total_trades: backtest_result.total_trades,
                }
            })
            .collect();

        results
    }

    pub fn sweep_sma_periods_with_stops(
        &self,
        fast_range: (usize, usize),
        slow_range: (usize, usize),
        step: usize,
        stop_loss: StopLossMethod,
    ) -> Vec<OptimizationResult> {
        let fast_periods: Vec<usize> = (fast_range.0..=fast_range.1).step_by(step).collect();
        let slow_periods: Vec<usize> = (slow_range.0..=slow_range.1).step_by(step).collect();

        let mut parameter_combinations = Vec::new();
        for fast in &fast_periods {
            for slow in &slow_periods {
                if fast < slow {
                    parameter_combinations.push((*fast, *slow));
                }
            }
        }

        let results: Vec<OptimizationResult> = parameter_combinations
            .par_iter()
            .map(|(fast, slow)| {
                let strategy = SMACrossover::new(*fast, *slow);
                let engine = BacktestEngine::new(
                    self.data.clone(),
                    self.initial_capital,
                    self.execution_model.clone(),
                )
                .with_stop_loss(stop_loss.clone());

                let backtest_result = engine.run(&strategy);

                OptimizationResult {
                    fast_period: *fast,
                    slow_period: *slow,
                    total_return: backtest_result.total_return,
                    sharpe_ratio: backtest_result.sharpe_ratio,
                    max_drawdown: backtest_result.max_drawdown,
                    total_trades: backtest_result.total_trades,
                }
            })
            .collect();

        results
    }

    pub fn save_results(results: &[OptimizationResult], path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(results)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn find_best_sharpe(results: &[OptimizationResult]) -> Option<&OptimizationResult> {
        results.iter().max_by(|a, b| {
            a.sharpe_ratio
                .partial_cmp(&b.sharpe_ratio)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn find_best_return(results: &[OptimizationResult]) -> Option<&OptimizationResult> {
        results.iter().max_by(|a, b| {
            a.total_return
                .partial_cmp(&b.total_return)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}
