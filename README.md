# StrataQuant v0.4.0

Truth in crypto backtesting. Risk management framework with position sizing, stop losses, and honest out-of-sample validation.

## What This Is

StrataQuant is a quantitative research engine for Bitcoin backtesting that prioritizes honesty over presentation. This tool shows real risk metrics, includes execution costs, and doesn't cherry-pick results.

**v0.4.0 adds:**
- Position sizing (Kelly, Fixed %, Fixed $, Half Kelly, Fixed Fractional)
- Stop losses (Fixed %, Trailing %, ATR-based, Time limit)
- Risk limits (drawdown thresholds, position size limits, trade frequency)
- **CRITICAL BUG FIX:** Proper sell execution (v0.3.0 didn't actually sell positions)

**v0.3.0 added:**
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
- **Proper trade execution** (v0.4.0 fix - v0.3.0 results were incorrect)

The v0.1.0 buy-and-hold result of 857% return demonstrates the opportunity. The -76.64% drawdown demonstrates the risk. Both numbers matter equally.

## Important Notice - v0.4.0 Bug Fix

**v0.3.0 had a critical bug:** The engine recorded trades but never executed sells. This inflated returns and made all results incorrect.

**v0.4.0 fixes this:** Proper buy AND sell execution. All results are now accurate.

**Impact:** v0.3.0 results are invalid. Always use v0.4.0 or later.

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

## Risk Management (v0.4.0 - Programmatic API)

```rust
use strataquant::backtest::{BacktestEngine, ExecutionModel, StopLossMethod, PositionSizingMethod};
use strataquant::data::load_from_parquet;
use strataquant::strategies::SMACrossover;
use std::path::Path;

fn main() {
    let data = load_from_parquet(Path::new("data/processed/btc_1d.parquet"))
        .expect("Failed to load data");
    
    let capital = 100_000.0;
    let execution_model = ExecutionModel::new(10.0, 5.0);
    let strategy = SMACrossover::new(20, 50);
    
    // Configure risk management
    let engine = BacktestEngine::new(data, capital, execution_model)
        .with_position_sizing(PositionSizingMethod::FixedPercent(50.0))
        .with_stop_loss(StopLossMethod::Trailing(10.0));
    
    let result = engine.run(&strategy);
    
    println!("Return: {:.2}%", result.total_return * 100.0);
    println!("Max DD: {:.2}%", result.max_drawdown * 100.0);
    println!("Trades: {}", result.total_trades);
}
```

See `src/bin/test_stops.rs` for complete examples.

**Note:** CLI support for risk management planned for v0.5.0.

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
│   │   ├── trade.rs       # Trade tracking (v0.3.0)
│   │   ├── position_sizing.rs  # Position sizing (v0.4.0)
│   │   ├── stops.rs       # Stop losses (v0.4.0)
│   │   └── risk.rs        # Risk limits (v0.4.0)
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

```bash
strataquant backtest --strategy buy-and-hold
```

**Expected results (Sept 2019 - Dec 2024, v0.4.0):**
- Return: ~857%
- Sharpe: ~0.83
- Max DD: ~-76%
- Trades: 2

### SMA Crossover
Golden cross / death cross system. Long when fast MA > slow MA.

```bash
strataquant backtest --strategy sma --fast 50 --slow 200
```

**Expected results (v0.4.0 - corrected):**
- Return: ~291%
- Sharpe: ~0.78
- Max DD: ~-47%
- Trades: 2

**Note:** v0.3.0 showed 949% but was incorrect due to sell execution bug.

### SMA 20/50 (More Active)
```bash
strataquant backtest --strategy sma --fast 20 --slow 50
```

**Expected results (v0.4.0):**
- Return: ~429%
- Sharpe: ~0.88
- Max DD: ~-37%
- Trades: 18

## Risk Management Results (v0.4.0)

### SMA 20/50 with Stop Losses

**No Stops (Baseline):**
- Return: 429%
- Max DD: -37%
- Trades: 18

**10% Trailing Stop:**
- Return: 418%
- Max DD: -36%
- Trades: 52 (stops triggered)

**5% Trailing Stop:**
- Return: 4% (too tight!)
- Max DD: -35%
- Trades: 46

**15% Fixed Stop:**
- Return: 427%
- Max DD: -37%
- Trades: 18 (rarely triggers)

**Conclusion:** 10% trailing stop provides modest risk reduction without killing returns. 5% is too tight for crypto volatility.

## Example Output

```
=== RESULTS ===
Initial capital: $   100000.00
Final equity:    $   529247.21
Total return:         429.25%
Sharpe ratio:            0.88
Sortino ratio:           1.30
Calmar ratio:            2.68
Max drawdown:         -37.37%
Total trades:              18

=== TRADE ANALYSIS ===
Win rate:               55.6%
Profit factor:           8.42
Avg win:         $   45234.12
Avg loss:        $    5378.92
Largest win:     $  187423.45
Largest loss:    $   12456.78
Expectancy:      $   23902.62
Win streak:                 4
Loss streak:                3
```

## Walk-Forward Validation

The honest part. Split data into train/test:

1. Optimize on 70% of data (in-sample)
2. Test on remaining 30% (out-of-sample)
3. Measure performance degradation

```bash
strataquant walkforward --train-ratio 0.7
```

**Expected results:**
- In-sample Sharpe: 1.05
- Out-of-sample Sharpe: 1.36
- Degradation: -29% (negative = better OOS!)

Note: Negative degradation means out-of-sample performed better, which occasionally happens but isn't reliable.

## Parameter Optimization

Grid search with parallel execution using rayon:

```bash
strataquant optimize --fast-range 20-100 --slow-range 50-200 --step 10
```

Tests all combinations where fast < slow. For the example above:
- Fast: 20, 30, 40, 50, 60, 70, 80, 90, 100
- Slow: 50, 60, 70, 80, 90, 100, 110, ..., 200
- Valid combinations: ~123

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
- Expect degradation out-of-sample
- Don't trust parameters optimized on full dataset
- Use optimization to understand parameter sensitivity

## Position Sizing Methods (v0.4.0)

**Fixed Percent:**
```rust
PositionSizingMethod::FixedPercent(50.0)  // 50% of equity
```

**Fixed Dollar:**
```rust
PositionSizingMethod::FixedDollar(50000.0)  // $50k per trade
```

**Kelly Criterion:**
```rust
PositionSizingMethod::Kelly {
    win_rate: 0.6,
    avg_win: 1000.0,
    avg_loss: 500.0,
}
```

**Half Kelly (Conservative):**
```rust
PositionSizingMethod::HalfKelly {
    win_rate: 0.6,
    avg_win: 1000.0,
    avg_loss: 500.0,
}
```

## Stop Loss Methods (v0.4.0)

**Fixed Percent:**
```rust
StopLossMethod::FixedPercent(10.0)  // 10% from entry
```

**Trailing Stop:**
```rust
StopLossMethod::Trailing(10.0)  // 10% from highest
```

**ATR-based:**
```rust
StopLossMethod::ATR {
    multiplier: 2.0,
    period: 14,
}
```

**Time Limit:**
```rust
StopLossMethod::TimeLimit(100)  // Max 100 bars
```

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
// Programmatically:
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
**Size:** ~0.05 MB for 5 years of daily data

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

**v0.5.0 - CLI Integration:**
- Command-line flags for stop losses
- Command-line flags for position sizing
- Interactive risk limit configuration

**v0.6.0 - Advanced Features:**
- Profit targets
- Partial position exits
- Dynamic position sizing
- Multi-timeframe stops

**v0.7.0 - Multi-Asset:**
- ETH, SOL, other pairs
- Correlation analysis
- Portfolio optimization
- Rebalancing strategies

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

**Important:** v0.3.0 had a critical bug (no sell execution). Always use v0.4.0 or later for accurate results.

StrataQuant shows you honest metrics so you can make informed decisions. It does not provide trading advice.

## Version History

- **v0.4.0** - Risk management (position sizing, stop losses), critical bug fix
- **v0.3.0** - Advanced metrics (Sortino, Calmar), trade analysis (results invalid due to bug)
- **v0.2.0** - Multi-strategy framework, optimization, walk-forward validation
- **v0.1.0** - Initial release with buy-and-hold baseline
