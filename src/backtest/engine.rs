use crate::backtest::{BacktestResult, ExecutionModel, Portfolio};
use crate::data::OHLCV;
use crate::metrics::{calculate_max_drawdown, calculate_sharpe_ratio};
use crate::strategies::Strategy;

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

    /// Run backtest with any strategy that implements the Strategy trait
    pub fn run(&self, strategy: &dyn Strategy) -> BacktestResult {
        let signals = strategy.generate_signals(&self.data);

        let mut portfolio = Portfolio::new(self.initial_capital);
        let mut equity_curve = Vec::with_capacity(self.data.len());
        let mut prev_position = 0.0;

        for (i, bar) in self.data.iter().enumerate() {
            let target_position = signals[i];

            // Execute trades when position changes
            if (target_position - prev_position).abs() > 1e-6 {
                if target_position > prev_position {
                    // Buy
                    let amount_to_buy = target_position - prev_position;
                    let buy_price = self.execution_model.execute_market_buy(bar.close);
                    let btc_to_buy = (portfolio.cash * amount_to_buy) / buy_price;

                    if portfolio.cash >= btc_to_buy * buy_price {
                        portfolio.buy(btc_to_buy, buy_price, self.execution_model.commission_bps);
                    }
                } else if target_position < prev_position {
                    // Sell (simplified - would need sell method in Portfolio)
                    // For now, we only handle long-only strategies
                }

                prev_position = target_position;
            }

            let equity = portfolio.equity(bar.close);
            equity_curve.push(equity);
        }

        let final_equity = *equity_curve.last().unwrap();
        let total_return = (final_equity - self.initial_capital) / self.initial_capital;

        let returns: Vec<f64> = equity_curve
            .windows(2)
            .map(|w| (w[1] - w[0]) / w[0])
            .collect();

        let sharpe_ratio = calculate_sharpe_ratio(&returns, 252.0);
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

    /// Legacy method for backwards compatibility
    pub fn run_buy_and_hold(&self) -> BacktestResult {
        use crate::strategies::BuyAndHold;
        self.run(&BuyAndHold::new())
    }
}
