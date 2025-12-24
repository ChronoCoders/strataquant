use crate::backtest::trade::{Trade, TradeStats};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    pub initial_capital: f64,
    pub final_equity: f64,
    pub total_return: f64,
    pub equity_curve: Vec<f64>,
    pub total_trades: u32,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub calmar_ratio: f64,
    pub max_drawdown: f64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<Trade>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_stats: Option<TradeStats>,
}

impl BacktestResult {
    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn save_equity_to_csv(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut csv = String::from("bar,equity\n");
        for (i, equity) in self.equity_curve.iter().enumerate() {
            csv.push_str(&format!("{},{}\n", i, equity));
        }

        std::fs::write(path, csv)?;
        Ok(())
    }

    pub fn save_trades_to_csv(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if let Some(trades) = &self.trades {
            let mut csv = String::from("trade_num,entry_timestamp,exit_timestamp,entry_price,exit_price,position_size,pnl,pnl_pct,duration_days,is_win\n");
            
            for (i, trade) in trades.iter().enumerate() {
                csv.push_str(&format!(
                    "{},{},{},{:.2},{:.2},{:.8},{:.2},{:.4},{:.1},{}\n",
                    i + 1,
                    trade.entry_timestamp,
                    trade.exit_timestamp,
                    trade.entry_price,
                    trade.exit_price,
                    trade.position_size,
                    trade.pnl,
                    trade.pnl_pct * 100.0,
                    trade.duration_days(),
                    trade.is_win
                ));
            }

            std::fs::write(path, csv)?;
        }

        Ok(())
    }
}
