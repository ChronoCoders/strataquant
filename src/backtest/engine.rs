use crate::backtest::{BacktestResult, ExecutionModel, Portfolio, Trade, TradeStats};
use crate::data::OHLCV;
use crate::metrics::{
    calculate_calmar_ratio, calculate_max_drawdown, calculate_sharpe_ratio, calculate_sortino_ratio,
};
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
        let mut trades: Vec<Trade> = Vec::new();
        let mut entry_bar: Option<usize> = None;
        let mut entry_price_with_costs: Option<f64> = None;
        let mut position_size: Option<f64> = None;

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

                        // Track entry
                        entry_bar = Some(i);
                        entry_price_with_costs = Some(buy_price);
                        position_size = Some(btc_to_buy);
                    }
                } else if target_position < prev_position {
                    // Sell - close the trade
                    if let (Some(entry_idx), Some(entry_px), Some(pos_size)) =
                        (entry_bar, entry_price_with_costs, position_size)
                    {
                        let exit_price = bar.close; // Simplified: no slippage on exit for now

                        let mut trade = Trade::new(
                            self.data[entry_idx].timestamp,
                            bar.timestamp,
                            entry_px,
                            exit_price,
                            pos_size,
                        );
                        trade.duration_bars = i - entry_idx;

                        trades.push(trade);
                    }

                    // Reset tracking
                    entry_bar = None;
                    entry_price_with_costs = None;
                    position_size = None;
                }

                prev_position = target_position;
            }

            let equity = portfolio.equity(bar.close);
            equity_curve.push(equity);
        }

        // Close any open position at the end
        if let (Some(entry_idx), Some(entry_px), Some(pos_size)) =
            (entry_bar, entry_price_with_costs, position_size)
        {
            let last_bar = self.data.last().unwrap();
            let mut trade = Trade::new(
                self.data[entry_idx].timestamp,
                last_bar.timestamp,
                entry_px,
                last_bar.close,
                pos_size,
            );
            trade.duration_bars = self.data.len() - 1 - entry_idx;
            trades.push(trade);
        }

        let final_equity = *equity_curve.last().unwrap();
        let total_return = (final_equity - self.initial_capital) / self.initial_capital;

        let returns: Vec<f64> = equity_curve
            .windows(2)
            .map(|w| (w[1] - w[0]) / w[0])
            .collect();

        let sharpe_ratio = calculate_sharpe_ratio(&returns, 252.0);
        let sortino_ratio = calculate_sortino_ratio(&returns, 252.0);
        let max_drawdown = calculate_max_drawdown(&equity_curve);
        let calmar_ratio = calculate_calmar_ratio(total_return, max_drawdown, self.data.len());

        let trade_stats = if !trades.is_empty() {
            Some(TradeStats::from_trades(&trades))
        } else {
            None
        };

        BacktestResult {
            initial_capital: self.initial_capital,
            final_equity,
            total_return,
            equity_curve,
            total_trades: portfolio.total_trades,
            sharpe_ratio,
            sortino_ratio,
            calmar_ratio,
            max_drawdown,
            trades: Some(trades),
            trade_stats,
        }
    }

    /// Legacy method for backwards compatibility
    pub fn run_buy_and_hold(&self) -> BacktestResult {
        use crate::strategies::BuyAndHold;
        self.run(&BuyAndHold::new())
    }
}
