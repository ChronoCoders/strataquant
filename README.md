# StrataQuant v0.2.0

Truth in crypto backtesting. Multi-strategy framework with honest out-of-sample validation.

## What This Is

StrataQuant is a quantitative research engine for Bitcoin backtesting that prioritizes honesty over presentation. This tool shows real risk metrics, includes execution costs, and doesn't cherry-pick results.

**v0.2.0 adds:**
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
- Single unoptimized strategy as baseline

The v0.1.0 buy-and-hold result of 857% return demonstrates the opportunity. The -76.64% drawdown demonstrates the risk. Both numbers matter.

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
  -t, --strategy <NAME>   Strategy: buy-and-hold, sma (default: buy-and-hold)
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
│   │   └── result.rs      # Result serialization
│   ├── optimization/      # Parameter optimization
│   │   ├── sweep.rs       # Grid search
│   │   └── walkforward.rs # Walk-forward validation
│   ├── metrics/           # Performance metrics
│   │   ├── sharpe.rs      # Sharpe ratio
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

### Maximum Drawdown
Peak-to-trough decline: max((equity - peak) / peak)
- Always negative
- -50% means you lost half your equity at worst point
- Critical for position sizing

## Strategy Implementations

### Buy and Hold
Baseline benchmark. Buy at first bar, hold until last bar.

```rust
strataquant backtest --strategy buy-and-hold
```

**Expected results (Sept 2019 - Dec 2024):**
- Return: ~857%
- Sharpe: ~0.99
- Max DD: ~-76%

### SMA Crossover
Golden cross / death cross system. Long when fast MA > slow MA.

```rust
strataquant backtest --strategy sma --fast 50 --slow 200
```

**Typical behavior:**
- Reduces drawdowns vs buy-and-hold
- Lower returns due to whipsaws
- Performance highly sensitive to parameters

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

**v0.3.0 - Advanced Metrics:**
- Calmar ratio
- Sortino ratio
- Win rate / profit factor
- Trade analysis (avg win, avg loss, longest streak)

**v0.4.0 - Risk Management:**
- Position sizing (Kelly, fixed fractional)
- Stop losses
- Portfolio heat limits

**v0.5.0 - Multi-Asset:**
- ETH, SOL, other pairs
- Correlation analysis
- Portfolio optimization

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
