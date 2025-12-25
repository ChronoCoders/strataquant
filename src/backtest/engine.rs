use crate::backtest::{
    calculate_atr, BacktestResult, ExecutionModel, Portfolio, PositionSizingMethod, RiskLimits,
    RiskMetrics, StopLossMethod, Trade, TradeStats,
};
use crate::data::OHLCV;
use crate::metrics::{
    calculate_calmar_ratio, calculate_max_drawdown, calculate_sharpe_ratio, calculate_sortino_ratio,
};
use crate::strategies::Strategy;

pub struct BacktestEngine {
    data: Vec<OHLCV>,
    initial_capital: f64,
    execution_model: ExecutionModel,
    position_sizing: PositionSizingMethod,
    stop_loss: StopLossMethod,
    risk_limits: RiskLimits,
}

impl BacktestEngine {
    pub fn new(data: Vec<OHLCV>, initial_capital: f64, execution_model: ExecutionModel) -> Self {
        Self {
            data,
            initial_capital,
            execution_model,
            position_sizing: PositionSizingMethod::default(),
            stop_loss: StopLossMethod::default(),
            risk_limits: RiskLimits::default(),
        }
    }

    pub fn with_position_sizing(mut self, method: PositionSizingMethod) -> Self {
        self.position_sizing = method;
        self
    }

    pub fn with_stop_loss(mut self, method: StopLossMethod) -> Self {
        self.stop_loss = method;
        self
    }

    pub fn with_risk_limits(mut self, limits: RiskLimits) -> Self {
        self.risk_limits = limits;
        self
    }

    pub fn run(&self, strategy: &dyn Strategy) -> BacktestResult {
        let signals = strategy.generate_signals(&self.data);

        let mut portfolio = Portfolio::new(self.initial_capital);
        let mut equity_curve = Vec::with_capacity(self.data.len());
        let mut prev_position = 0.0;
        let mut trades: Vec<Trade> = Vec::new();
        let mut entry_bar: Option<usize> = None;
        let mut entry_price_with_costs: Option<f64> = None;
        let mut position_size: Option<f64> = None;
        let mut highest_price: f64 = 0.0;
        let mut risk_metrics = RiskMetrics::new(self.initial_capital);

        // Pre-calculate ATR if needed
        let atr_values = if matches!(self.stop_loss, StopLossMethod::ATR { .. }) {
            let hlc_data: Vec<(f64, f64, f64)> = self
                .data
                .iter()
                .map(|bar| (bar.high, bar.low, bar.close))
                .collect();

            let period = match self.stop_loss {
                StopLossMethod::ATR { period, .. } => period,
                _ => 14,
            };

            calculate_atr(&hlc_data, period)
        } else {
            vec![f64::NAN; self.data.len()]
        };

        for (i, bar) in self.data.iter().enumerate() {
            let target_position = signals[i];

            // Check for stop loss exit
            let mut stop_hit = false;
            if let (Some(entry_idx), Some(entry_px), Some(_pos_size)) =
                (entry_bar, entry_price_with_costs, position_size)
            {
                let bars_held = i - entry_idx;
                let atr = if atr_values[i].is_nan() {
                    None
                } else {
                    Some(atr_values[i])
                };

                if self
                    .stop_loss
                    .is_hit(entry_px, bar.close, highest_price, bars_held, atr)
                {
                    stop_hit = true;
                }

                // Update highest price for trailing stop
                if bar.close > highest_price {
                    highest_price = bar.close;
                }
            }

            // Execute stop loss exit
            if stop_hit {
                if let (Some(entry_idx), Some(entry_px), Some(pos_size)) =
                    (entry_bar, entry_price_with_costs, position_size)
                {
                    let sell_price = self.execution_model.execute_market_sell(bar.close);

                    // Actually sell the BTC
                    portfolio.sell(pos_size, sell_price, self.execution_model.commission_bps);

                    let mut trade = Trade::new(
                        self.data[entry_idx].timestamp,
                        bar.timestamp,
                        entry_px,
                        sell_price,
                        pos_size,
                    );
                    trade.duration_bars = i - entry_idx;

                    trades.push(trade);
                    risk_metrics.on_trade();

                    // Reset tracking
                    entry_bar = None;
                    entry_price_with_costs = None;
                    position_size = None;
                    highest_price = 0.0;
                    prev_position = 0.0;
                }
            }

            // Execute trades when position changes (strategy signal)
            if !stop_hit && (target_position - prev_position).abs() > 1e-6 {
                if target_position > prev_position {
                    // Buy
                    let equity = portfolio.equity(bar.close);

                    // Check risk limits before entering
                    if risk_metrics.risk_limit_violations == 0
                        && self
                            .risk_limits
                            .check_drawdown(equity, risk_metrics.peak_equity)
                        && self.risk_limits.can_trade(
                            risk_metrics.bars_since_last_trade,
                            risk_metrics.trades_today,
                        )
                    {
                        // Calculate position size
                        let position_value = self.position_sizing.calculate_size(equity, None);

                        if self.risk_limits.check_position_size(position_value, equity) {
                            let buy_price = self.execution_model.execute_market_buy(bar.close);
                            let btc_to_buy = position_value / buy_price;

                            if portfolio.cash >= btc_to_buy * buy_price {
                                portfolio.buy(
                                    btc_to_buy,
                                    buy_price,
                                    self.execution_model.commission_bps,
                                );

                                entry_bar = Some(i);
                                entry_price_with_costs = Some(buy_price);
                                position_size = Some(btc_to_buy);
                                highest_price = bar.close;

                                risk_metrics.on_trade();
                            }
                        } else {
                            risk_metrics.risk_limit_violations += 1;
                        }
                    } else {
                        risk_metrics.risk_limit_violations += 1;
                    }
                } else if target_position < prev_position {
                    // Sell - close the trade
                    if let (Some(entry_idx), Some(entry_px), Some(pos_size)) =
                        (entry_bar, entry_price_with_costs, position_size)
                    {
                        let sell_price = self.execution_model.execute_market_sell(bar.close);

                        // Actually sell the BTC
                        portfolio.sell(pos_size, sell_price, self.execution_model.commission_bps);

                        let mut trade = Trade::new(
                            self.data[entry_idx].timestamp,
                            bar.timestamp,
                            entry_px,
                            sell_price,
                            pos_size,
                        );
                        trade.duration_bars = i - entry_idx;

                        trades.push(trade);
                        risk_metrics.on_trade();
                    }

                    // Reset tracking
                    entry_bar = None;
                    entry_price_with_costs = None;
                    position_size = None;
                    highest_price = 0.0;
                }

                prev_position = target_position;
            }

            let equity = portfolio.equity(bar.close);
            equity_curve.push(equity);

            // Update risk metrics
            let exposure = if let Some(pos_size) = position_size {
                pos_size * bar.close
            } else {
                0.0
            };
            risk_metrics.update(equity, exposure);
        }

        // Close any open position at the end
        if let (Some(entry_idx), Some(entry_px), Some(pos_size)) =
            (entry_bar, entry_price_with_costs, position_size)
        {
            let last_bar = self.data.last().unwrap();
            let sell_price = self.execution_model.execute_market_sell(last_bar.close);

            // Actually sell the BTC
            portfolio.sell(pos_size, sell_price, self.execution_model.commission_bps);

            let mut trade = Trade::new(
                self.data[entry_idx].timestamp,
                last_bar.timestamp,
                entry_px,
                sell_price,
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

    pub fn run_buy_and_hold(&self) -> BacktestResult {
        use crate::strategies::BuyAndHold;
        self.run(&BuyAndHold::new())
    }
}
