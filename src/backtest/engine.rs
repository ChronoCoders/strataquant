use crate::backtest::result::BacktestResult;
use crate::backtest::types::{ExecutionModel, Portfolio};
use crate::data::OHLCV;
use crate::metrics::{calculate_max_drawdown, calculate_sharpe_ratio};

pub struct BacktestEngine {
    data: Vec<OHLCV>,
    initial_capital: f64,
    execution_model: ExecutionModel,
}

impl BacktestEngine {
    pub fn new(data: Vec<OHLCV>, initial_capital: f64, execution_model: ExecutionModel) -> Self {
        Self {
            data,
            initial_capital,
            execution_model,
        }
    }

    pub fn run_buy_and_hold(&self) -> BacktestResult {
        let mut portfolio = Portfolio::new(self.initial_capital);
        let mut equity_curve = Vec::new();

        let first_bar = &self.data[0];
        let execution_price = self.execution_model.execute_market_buy(first_bar.close);
        let btc_to_buy = portfolio.cash / execution_price;

        portfolio.buy(
            btc_to_buy,
            execution_price,
            self.execution_model.commission_bps,
        );

        for bar in &self.data {
            let equity = portfolio.equity(bar.close);
            equity_curve.push(equity);
        }

        let final_equity = equity_curve[equity_curve.len() - 1];
        let total_return = (final_equity - self.initial_capital) / self.initial_capital;

        let returns: Vec<f64> = equity_curve
            .windows(2)
            .map(|w| (w[1] - w[0]) / w[0])
            .collect();

        let sharpe_ratio = calculate_sharpe_ratio(&returns, 365.0);
        let max_drawdown = calculate_max_drawdown(&equity_curve);

        BacktestResult {
            initial_capital: self.initial_capital,
            final_equity,
            total_return,
            equity_curve,
            total_trades: portfolio.total_trades,
            sharpe_ratio,
            max_drawdown,
        }
    }
}
