# StrataQuant v0.5.0

Truth in crypto backtesting. Production-grade backtesting engine with complete CLI risk management support.

## What This Is

StrataQuant is a quantitative research engine for Bitcoin backtesting that prioritizes honesty over presentation. This tool shows real risk metrics, includes execution costs, and doesn't cherry-pick results.

**v0.5.0 adds:**
- Full CLI integration for risk management
- Stop losses for all commands (backtest, optimize, walkforward, compare)
- Position sizing controls via command line
- Risk limit configuration
- Zero clippy warnings (clean, maintainable code)

**v0.4.0 added:**
- Stop loss methods (fixed, trailing, ATR, time-based)
- Position sizing (fixed percent, fixed dollar)
- Risk limits (max drawdown, position size controls)
- Builder pattern for risk configuration

**v0.3.0 added:**
- Sortino ratio (downside deviation only)
- Calmar ratio (return per unit of drawdown)
- Complete trade tracking and analysis
- Win rate, profit factor, expectancy

**v0.2.0 added:**
- Multi-strategy framework with pluggable Strategy trait
- SMA crossover implementation
- Parameter optimization with parallel execution
- Walk-forward validation to detect overfitting

## Philosophy

Most backtesting engines show you inflated returns and hide the drawdowns. StrataQuant does the opposite:

- Shows real drawdowns (-76.64% for buy-and-hold)
- Includes execution costs (15 bps total)
- No parameter cherry-picking
- Walk-forward validation exposes overfitting
- Complete trade analysis with win rates and streaks
- Advanced risk metrics (Sortino, Calmar)
- **Professional risk management (NEW in v0.4.0/v0.5.0)**

The v0.1.0 buy-and-hold result of 857% return demonstrates the opportunity. The -76.64% drawdown demonstrates the risk. Both numbers matter equally.

## Installation

```bash
# Clone the repository
git clone https://github.com/ChronoCoders/strataquant.git
cd strataquant

# Build release binary
cargo build --release

# Binary location
./target/release/strataquant
```

## Quick Start

```bash
# 1. Download historical data
strataquant download --start 2019-09-08 --end 2024-12-22

# 2. Run buy-and-hold backtest
strataquant backtest

# 3. Test SMA crossover with trailing stop
strataquant backtest --strategy sma --fast 20 --slow 50 \
  --stop-type trailing --stop-pct 10.0

# 4. Optimize with ATR stops
strataquant optimize --fast-range 20-50 --slow-range 50-100 \
  --stop-type atr --atr-multiplier 2.0

# 5. Walk-forward validation with stops
strataquant walkforward --train-ratio 0.7 \
  --stop-type trailing --stop-pct 10.0

# 6. Compare strategies with risk controls
strataquant compare --stop-type trailing --stop-pct 5.0
```

## Commands

### download
Download historical BTC/USDT data from Binance.US

```bash
strataquant download [OPTIONS]

Options:
  -s, --start <DATE>      Start date (default: 2019-09-08)
  -e, --end <DATE>        End date (default: 2024-12-22)
  -i, --interval <INT>    Interval: 1d, 1h, 5m, 1m (default: 1d)
```

### backtest
Run backtest with specified strategy and risk controls

```bash
strataquant backtest [OPTIONS]

Strategy Options:
  -t, --strategy <n>          Strategy: buy-and-hold, sma (default: buy-and-hold)
  -f, --fast <PERIOD>         Fast SMA period (default: 50)
  -w, --slow <PERIOD>         Slow SMA period (default: 200)

Execution Options:
  -c, --capital <USD>         Initial capital (default: 100000)
  -m, --commission <BPS>      Commission in basis points (default: 10)
  -l, --slippage <BPS>        Slippage in basis points (default: 5)

Risk Management (NEW in v0.5.0):
  --stop-type <type>          Stop loss: none, fixed, trailing, atr, time (default: none)
  --stop-pct <percent>        Stop percentage for fixed/trailing (default: 10.0)
  --atr-multiplier <mult>     ATR multiplier for atr stops (default: 2.0)
  --atr-period <period>       ATR period (default: 14)
  --time-limit <bars>         Time limit for time stops (default: 100)
  --position-sizing <type>    Position sizing: fixed-pct, fixed-dollar (default: fixed-pct)
  --position-size <value>     Position size value (default: 100.0)
  --max-drawdown <percent>    Max drawdown threshold (default: 50.0)
```

**Examples:**

```bash
# Basic backtest
strataquant backtest --strategy sma --fast 20 --slow 50

# With 10% trailing stop
strataquant backtest --strategy sma --fast 20 --slow 50 \
  --stop-type trailing --stop-pct 10.0

# With ATR stop
strataquant backtest --strategy sma --fast 20 --slow 50 \
  --stop-type atr --atr-multiplier 2.0 --atr-period 14

# With 50% position sizing
strataquant backtest --strategy sma --fast 20 --slow 50 \
  --position-sizing fixed-pct --position-size 50.0

# Conservative: 50% size + 10% trailing stop + 30% DD limit
strataquant backtest --strategy sma --fast 20 --slow 50 \
  --position-sizing fixed-pct --position-size 50.0 \
  --stop-type trailing --stop-pct 10.0 \
  --max-drawdown 30.0
```

### optimize
Grid search over parameter space with risk controls

```bash
strataquant optimize [OPTIONS]

Options:
  --fast-range <min-max>      Fast period range (default: 20-100)
  --slow-range <min-max>      Slow period range (default: 50-200)
  --step <size>               Step size (default: 10)
  -c, --capital <USD>         Initial capital (default: 100000)
  -m, --commission <BPS>      Commission in basis points (default: 10)
  -l, --slippage <BPS>        Slippage in basis points (default: 5)
  
Risk Management (NEW in v0.5.0):
  --stop-type <type>          Stop loss: none, fixed, trailing, atr (default: none)
  --stop-pct <percent>        Stop percentage (default: 10.0)
  --atr-multiplier <mult>     ATR multiplier (default: 2.0)
  --atr-period <period>       ATR period (default: 14)
```

### walkforward
Walk-forward validation with risk controls

```bash
strataquant walkforward [OPTIONS]

Options:
  --train-ratio <ratio>       Train/test split (default: 0.7)
  -c, --capital <USD>         Initial capital (default: 100000)
  -m, --commission <BPS>      Commission in basis points (default: 10)
  -l, --slippage <BPS>        Slippage in basis points (default: 5)

Risk Management (NEW in v0.5.0):
  --stop-type <type>          Stop loss: none, fixed, trailing, atr (default: none)
  --stop-pct <percent>        Stop percentage (default: 10.0)
  --atr-multiplier <mult>     ATR multiplier (default: 2.0)
  --atr-period <period>       ATR period (default: 14)
```

### compare
Compare all strategies side-by-side with risk controls

```bash
strataquant compare [OPTIONS]

Options:
  -c, --capital <USD>         Initial capital (default: 100000)
  -m, --commission <BPS>      Commission in basis points (default: 10)
  -l, --slippage <BPS>        Slippage in basis points (default: 5)

Risk Management (NEW in v0.5.0):
  --stop-type <type>          Stop loss: none, fixed, trailing, atr (default: none)
  --stop-pct <percent>        Stop percentage (default: 10.0)
  --atr-multiplier <mult>     ATR multiplier (default: 2.0)
  --atr-period <period>       ATR period (default: 14)
```

## Results (Sept 2019 - Dec 2024)

### Buy and Hold
```
Initial capital: $100,000.00
Final equity:    $957,307.75
Total return:         857.31%
Sharpe ratio:            0.83
Sortino ratio:           1.21
Calmar ratio:            2.13
Max drawdown:         -76.64%
Total trades:               2
Win rate:              100.0%
```

### SMA 20/50
```
Initial capital: $100,000.00
Final equity:    $529,247.21
Total return:         429.25%
Sharpe ratio:            0.88
Sortino ratio:           1.43
Calmar ratio:            2.19
Max drawdown:         -37.37%
Total trades:              18
Win rate:               44.4%
Profit factor:           3.27
```

### SMA 20/50 with 10% Trailing Stop (NEW)
```
Initial capital: $100,000.00
Final equity:    $518,260.37
Total return:         418.26%
Sharpe ratio:            0.88
Sortino ratio:           1.43
Calmar ratio:            2.18
Max drawdown:         -36.49%
Total trades:              52
Win rate:               57.7%
Profit factor:           2.24
```

### SMA 20/50 with ATR Stop (2x14) (NEW)
```
Initial capital: $100,000.00
Final equity:    $563,491.18
Total return:         463.49%
Sharpe ratio:            0.91
Sortino ratio:           1.49
Calmar ratio:            2.67
Max drawdown:         -33.09%
Total trades:              18
Win rate:               44.4%
Profit factor:           3.98
```

**Key Finding:** ATR stops outperform fixed percentage stops for crypto volatility.

## Metrics Explained

### Returns
- **Total Return**: (final_equity - initial_capital) / initial_capital
- Includes all commissions and slippage

### Risk Metrics
- **Sharpe Ratio**: (mean_return / volatility) × √252
  - Measures return per unit of volatility
  - Above 1.0 is good
  
- **Sortino Ratio**: (mean_return / downside_deviation) × √252
  - Like Sharpe but only penalizes downside volatility
  - Better for asymmetric strategies
  
- **Calmar Ratio**: annualized_return / absolute_max_drawdown
  - Return per unit of worst drawdown
  - Above 2.0 is good

- **Max Drawdown**: Largest peak-to-trough decline
  - Critical for position sizing
  - Shows worst-case scenario

### Trade Stats
- **Win Rate**: % of profitable trades
- **Profit Factor**: total_wins / total_losses
- **Expectancy**: Average $ per trade

## Risk Management (v0.4.0/v0.5.0)

### Stop Loss Methods

**Fixed Percent:**
```bash
--stop-type fixed --stop-pct 10.0
```

**Trailing:**
```bash
--stop-type trailing --stop-pct 10.0
```

**ATR-Based (Best for crypto):**
```bash
--stop-type atr --atr-multiplier 2.0 --atr-period 14
```

**Time-Based:**
```bash
--stop-type time --time-limit 100
```

### Position Sizing

**Fixed Percent:**
```bash
--position-sizing fixed-pct --position-size 100.0  # All-in
--position-sizing fixed-pct --position-size 50.0   # Half
```

**Fixed Dollar:**
```bash
--position-sizing fixed-dollar --position-size 10000
```

## Programmatic API

```rust
use strataquant::backtest::{BacktestEngine, ExecutionModel, StopLossMethod};
use strataquant::strategies::SMACrossover;

let strategy = SMACrossover::new(20, 50);
let execution_model = ExecutionModel::new(10.0, 5.0);

let engine = BacktestEngine::new(data, 100_000.0, execution_model)
    .with_stop_loss(StopLossMethod::Trailing(10.0));

let result = engine.run(&strategy);
println!("Return: {:.2}%", result.total_return * 100.0);
```

## Roadmap

**✓ v0.5.0 - CLI Integration (COMPLETE):**
- Full CLI risk management support
- All commands support stops
- Zero clippy warnings

**v0.6.0 - Advanced Features:**
- Kelly criterion position sizing
- Volatility-based position sizing
- Multiple timeframe support
- Advanced indicator library (RSI, MACD, Bollinger Bands)
- ~6-8 hours of development

**v0.7.0 - Multi-Asset:**
- Portfolio support: BTC, ETH, LTC, BCH, SOL
- Correlation analysis
- Portfolio optimization
- Asset allocation strategies
- ~10-12 hours of development

## Development

```bash
# Run tests
cargo test

# Check for warnings
cargo clippy

# Format code
cargo fmt
```

## License

MIT

## Contact

Built by Distributed Systems Labs, LLC
- GitHub: [@ChronoCoders](https://github.com/ChronoCoders)
- Website: https://dslabs.network

## Disclaimer

This software is for educational and research purposes only. Past performance does not guarantee future results. Trading cryptocurrencies carries significant risk. Only trade with capital you can afford to lose.

StrataQuant shows you honest metrics so you can make informed decisions. It does not provide trading advice.

## Version History

- **v0.5.0** - CLI integration for risk management (all commands support stops)
- **v0.4.0** - Risk management system (stops, position sizing, risk limits)
- **v0.3.0** - Advanced metrics (Sortino, Calmar), complete trade analysis
- **v0.2.0** - Multi-strategy framework, optimization, walk-forward validation
- **v0.1.0** - Initial release with buy-and-hold baseline
