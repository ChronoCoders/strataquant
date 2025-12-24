# StrataQuant v0.3.0

Truth in crypto backtesting. Multi-strategy framework with advanced metrics and honest out-of-sample validation.

## What This Is

StrataQuant is a quantitative research engine for Bitcoin backtesting that prioritizes honesty over presentation. This tool shows real risk metrics, includes execution costs, and doesn't cherry-pick results.

**v0.3.0 adds:**
- Sortino ratio (downside deviation only)
- Calmar ratio (return per unit of drawdown)
- Complete trade tracking and analysis
- Win rate, profit factor, expectancy
- Win/loss streak tracking
- CSV export for trades and equity curves

**v0.2.0 added:**
- Multi-strategy framework with pluggable Strategy trait
- SMA crossover implementation
- Parameter optimization with parallel execution
- Walk-forward validation to detect overfitting
- Strategy comparison tools

## Philosophy

Most backtesting engines show you inflated returns and hide the drawdowns. StrataQuant does the opposite:

- Shows real drawdowns (-76.64% for buy-and-hold)
- Includes execution costs (15 bps total)
- No parameter cherry-picking
- Walk-forward validation exposes overfitting
- Complete trade analysis with win rates and streaks
- Advanced risk metrics (Sortino, Calmar)

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

# 3. Test SMA crossover
strataquant backtest --strategy sma --fast 50 --slow 200

# 4. Optimize parameters
strataquant optimize --fast-range 20-100 --slow-range 50-200 --step 10

# 5. Walk-forward validation
strataquant walkforward --train-ratio 0.7

# 6. Compare all strategies
strataquant compare
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
Run backtest with specified strategy

```bash
strataquant backtest [OPTIONS]

Options:
  -t, --strategy <n>      Strategy: buy-and-hold, sma (default: buy-and-hold)
  -f, --fast <PERIOD>     Fast SMA period (default: 50)
  -w, --slow <PERIOD>     Slow SMA period (default: 200)
  -c, --capital <USD>     Initial capital (default: 100000)
  -m, --commission <BPS>  Commission in basis points (default: 10)
  -l, --slippage <BPS>    Slippage in basis points (default: 5)
```

### optimize
Grid search over SMA parameter space

```bash
strataquant optimize [OPTIONS]

Options:
  --fast-range <MIN-MAX>  Fast period range (default: 20-100)
  --slow-range <MIN-MAX>  Slow period range (default: 50-200)
  --step <SIZE>           Step size (default: 10)
  -c, --capital <USD>     Initial capital (default: 100000)
  -m, --commission <BPS>  Commission in basis points (default: 10)
  -l, --slippage <BPS>    Slippage in basis points (default: 5)
```

### walkforward
Walk-forward validation to detect overfitting

```bash
strataquant walkforward [OPTIONS]

Options:
  --train-ratio <RATIO>   Train/test split (default: 0.7)
  -c, --capital <USD>     Initial capital (default: 100000)
  -m, --commission <BPS>  Commission in basis points (default: 10)
  -l, --slippage <BPS>    Slippage in basis points (default: 5)
```

### compare
Compare all implemented strategies

```bash
strataquant compare [OPTIONS]

Options:
  -c, --capital <USD>     Initial capital (default: 100000)
  -m, --commission <BPS>  Commission in basis points (default: 10)
  -l, --slippage <BPS>    Slippage in basis points (default: 5)
```

## Architecture

```
strataquant/
├── src/
│   ├── data/              # Data download and storage
│   │   ├── binance.rs     # Binance.US API client
│   │   ├── storage.rs     # Parquet I/O
│   │   └── types.rs       # OHLCV data structure
│   ├── strategies/        # Trading strategies
│   │   ├── trait.rs       # Strategy trait definition
│   │   ├── buy_and_hold.rs
│   │   └── sma_crossover.rs
│   ├── backtest/          # Backtesting engine
│   │   ├── engine.rs      # Core backtest logic
│   │   ├── types.rs       # Portfolio & execution
│   │   ├── result.rs      # Result serialization
│   │   └── trade.rs       # Trade tracking (v0.3.0)
│   ├── optimization/      # Parameter optimization
│   │   ├── sweep.rs       # Grid search
│   │   └── walkforward.rs # Walk-forward validation
│   ├── metrics/           # Performance metrics
│   │   ├── sharpe.rs      # Sharpe ratio
│   │   ├── sortino.rs     # Sortino ratio (v0.3.0)
│   │   ├── calmar.rs      # Calmar ratio (v0.3.0)
│   │   └── drawdown.rs    # Maximum drawdown
│   ├── lib.rs             # Library exports
│   └── main.rs            # CLI interface
├── data/processed/        # Downloaded data (Parquet)
└── results/               # Backtest results (JSON)
```

## Metrics Explained

### Total Return
Simple return: (final_equity - initial_capital) / initial_capital

### Sharpe Ratio
Risk-adjusted return: (mean_return / std_dev) * sqrt(252)
- Above 1.0 is good
- Above 2.0 is excellent
- Above 3.0 is suspicious (check for overfitting)

### Sortino Ratio (v0.3.0)
Downside risk-adjusted return: (mean_return / downside_deviation) * sqrt(252)
- Only penalizes downside volatility (better for asymmetric strategies)
- Higher than Sharpe indicates positive skew
- Above 1.5 is good
- Above 2.5 is excellent

### Calmar Ratio (v0.3.0)
Return per unit of max drawdown: annualized_return / |max_drawdown|
- Measures return efficiency relative to worst loss
- Above 1.0 is acceptable
- Above 2.0 is good
- Above 3.0 is excellent

### Maximum Drawdown
Peak-to-trough decline: max((equity - peak) / peak)
- Always negative
- -50% means you lost half your equity at worst point
- Critical for position sizing

### Trade Analysis (v0.3.0)

**Win Rate**: Percentage of profitable trades
- 50%+ is baseline for trend-following
- 60%+ is good
- 70%+ is exceptional (or overfit)

**Profit Factor**: Total wins / Total losses
- 1.0 = break even
- 1.5+ = profitable
- 2.0+ = good
- 3.0+ = excellent

**Expectancy**: Average profit per trade
- Positive = profitable system
- Higher is better
- Accounts for win rate and avg win/loss

**Streaks**: Longest consecutive wins/losses
- Important for psychological resilience
- Longer loss streaks require more capital reserves

## Strategy Implementations

### Buy and Hold
Baseline benchmark. Buy at first bar, hold until last bar.

```rust
strataquant backtest --strategy buy-and-hold
```

**Expected results (Sept 2019 - Dec 2024):**
- Return: ~857%
- Sharpe: ~0.83
- Sortino: ~1.21
- Calmar: ~2.13
- Max DD: ~-76%
- Win rate: 100% (single trade)

### SMA Crossover
Golden cross / death cross system. Long when fast MA > slow MA.

```rust
strataquant backtest --strategy sma --fast 50 --slow 200
```

**Typical behavior:**
- Reduces whipsaws vs faster MAs
- Better Sortino than Sharpe (asymmetric returns)
- Win rate typically 60-80%
- Profit factor 100+ (small losses, big wins)

## Example Output

```
=== RESULTS ===
Initial capital: $   100000.00
Final equity:    $  1049354.30
Total return:         949.35%
Sharpe ratio:            0.89
Sortino ratio:           1.34
Calmar ratio:            2.36
Max drawdown:         -76.63%
Total trades:               5

=== TRADE ANALYSIS ===
Win rate:               80.0%
Profit factor:         432.67
Avg win:         $   72910.70
Avg loss:        $       2.89
Largest win:     $  291632.26
Largest loss:    $      28.88
Expectancy:      $   58328.56
Win streak:                 3
Loss streak:                1
```

## Walk-Forward Validation

The honest part. Split data into train/test:

1. Optimize on 70% of data (in-sample)
2. Test on remaining 30% (out-of-sample)
3. Measure performance degradation

```rust
strataquant walkforward --train-ratio 0.7
```

**Expected results:**
- In-sample Sharpe: 1.5-2.0
- Out-of-sample Sharpe: 0.4-0.8
- Degradation: 50-80%

This degradation is normal. It proves the optimization fit to noise, not signal.

## Parameter Optimization

Grid search with parallel execution using rayon:

```rust
strataquant optimize --fast-range 20-100 --slow-range 50-200 --step 10
```

Tests all combinations where fast < slow. For the example above:
- Fast: 20, 30, 40, 50, 60, 70, 80, 90, 100
- Slow: 50, 60, 70, 80, 90, 100, 110, ..., 200
- Valid combinations: ~150

**Results saved to:** `results/optimization_results.json`

## The Truth About Optimization

Parameter optimization finds the best parameters for historical data. This does NOT mean they'll work in the future.

**Why optimization fails:**
1. Overfitting to noise
2. Non-stationary markets (what worked stops working)
3. Transaction costs eat the edge
4. Survivorship bias

**How to use optimization honestly:**
- Always use walk-forward validation
- Expect 50%+ degradation out-of-sample
- Don't trust parameters optimized on full dataset
- Use optimization to understand parameter sensitivity

## Trade Analysis Features (v0.3.0)

Every backtest now tracks individual trades:

**Captured Data:**
- Entry/exit timestamps and prices
- Position size
- Profit/loss ($ and %)
- Duration (bars and days)
- Win/loss classification

**Calculated Statistics:**
- Win rate (% profitable trades)
- Profit factor (wins/losses ratio)
- Average win and loss sizes
- Largest win and loss
- Expectancy (expected value per trade)
- Longest winning/losing streaks

**Export Options:**
```rust
// In your code, programmatically:
result.save_trades_to_csv(Path::new("results/trades.csv"))?;
result.save_equity_to_csv(Path::new("results/equity.csv"))?;
```

## Adding New Strategies

Implement the `Strategy` trait:

```rust
use crate::data::OHLCV;
use crate::strategies::Strategy;

pub struct MyStrategy {
    // Strategy parameters
}

impl Strategy for MyStrategy {
    fn generate_signals(&self, data: &[OHLCV]) -> Vec<f64> {
        // Return position vector: 1.0 = long, 0.0 = flat
        vec![1.0; data.len()]
    }

    fn name(&self) -> &str {
        "My Strategy"
    }

    fn description(&self) -> String {
        "Description of what this strategy does".to_string()
    }
}
```

Register in `src/strategies/mod.rs` and add to CLI.

## Data

**Source:** Binance.US API (free, no auth required)
**Format:** Parquet with snappy compression
**Size:** ~40 MB for 5 years of daily data

Data includes:
- Timestamp (UTC milliseconds)
- Open, High, Low, Close prices
- Volume

## Execution Model

**Commission:** 10 bps (0.10%) per trade
**Slippage:** 5 bps (0.05%) on market orders
**Total cost:** 15 bps per round trip

This is realistic for retail crypto trading. Adjust for your actual costs.

## Roadmap

**v0.4.0 - Risk Management:**
- Position sizing (Kelly, fixed fractional)
- Stop losses (trailing, fixed)
- Portfolio heat limits
- Risk-adjusted position entry

**v0.5.0 - Multi-Asset:**
- ETH, SOL, other pairs
- Correlation analysis
- Portfolio optimization
- Rebalancing strategies

**v0.6.0 - Advanced Strategies:**
- Mean reversion
- Breakout detection
- Multiple timeframe analysis
- Machine learning integration

## Development

```bash
# Run tests
cargo test

# Check for warnings
cargo clippy

# Format code
cargo fmt

# Build docs
cargo doc --open
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

- **v0.3.0** - Advanced metrics (Sortino, Calmar), complete trade analysis
- **v0.2.0** - Multi-strategy framework, optimization, walk-forward validation
- **v0.1.0** - Initial release with buy-and-hold baseline
