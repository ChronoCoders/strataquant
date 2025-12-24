# StrataQuant v0.2.0 - Deployment Guide

Complete guide for building, testing, and using the multi-strategy backtesting framework.

## Prerequisites

**Required:**
- Rust 1.70+ (install from https://rustup.rs/)
- 8GB RAM minimum
- 1GB free disk space

**Optional:**
- Git for version control
- Multi-core CPU for faster optimization

## Installation

### Extract Archive

```bash
# Linux/Mac
tar -xzf strataquant-v0.2.0.tar.gz
cd strataquant

# Windows
# Extract using 7-Zip or Windows built-in extraction
cd strataquant
```

### Build from Source

**Linux/Mac:**
```bash
chmod +x build.sh
./build.sh
```

**Windows:**
```cmd
build.bat
```

Build time: 2-3 minutes on first build (downloads dependencies)

### Verify Installation

```bash
# Linux/Mac
./target/release/strataquant --help

# Windows
target\release\strataquant.exe --help
```

Expected output:
```
StrataQuant - Honest BTC Backtesting Engine

Usage: strataquant <COMMAND>

Commands:
  download     Download historical BTC data from Binance
  backtest     Run backtest on downloaded data
  optimize     Optimize SMA parameters with grid search
  walkforward  Walk-forward validation
  compare      Compare all strategies
  help         Print this message or the help of the given subcommand(s)
```

## Quick Start Workflow

### 1. Download Historical Data (~30 seconds)

```bash
strataquant download
```

This downloads BTC/USDT daily data from Binance.US (Sept 2019 - Dec 2024).

Output:
```
Downloaded 1919 candles
Saved to: data/processed/btc_1d.parquet
Success! File size: 0.04 MB
```

### 2. Run Buy-and-Hold Baseline (~1 second)

```bash
strataquant backtest
```

Expected results:
```
=== RESULTS ===
Initial capital: $  100,000.00
Final equity:    $  957,307.75
Total return:        857.31%
Sharpe ratio:           0.99
Max drawdown:         -76.64%
Total trades:              1
```

### 3. Test SMA Crossover (~1 second)

```bash
strataquant backtest --strategy sma --fast 50 --slow 200
```

This tests the classic 50/200 SMA crossover (golden cross / death cross).

### 4. Parameter Optimization (~30 seconds)

```bash
strataquant optimize
```

Tests 150+ parameter combinations in parallel.

Output shows best parameters by Sharpe ratio and return:
```
=== BEST BY SHARPE RATIO ===
Parameters: 60/170
Sharpe ratio: 1.45
Total return: 523.67%
Max drawdown: -48.23%
Total trades: 12
```

Results saved to: `results/optimization_results.json`

### 5. Walk-Forward Validation (~45 seconds)

```bash
strataquant walkforward
```

The critical test. Splits data 70/30, optimizes on train, validates on test.

Expected output:
```
=== WALK-FORWARD RESULTS ===
Optimal parameters: 70/180

In-Sample (Training):
  Return:         689.45%
  Sharpe ratio:      1.82

Out-of-Sample (Testing):
  Return:         156.23%
  Sharpe ratio:      0.67

Performance Degradation:
  Return:           77.34%
  Sharpe ratio:     63.19%

⚠️  WARNING: Significant degradation detected!
This suggests the optimization may have overfit to the training data.
```

This is honest validation. The degradation is expected and real.

### 6. Compare All Strategies (~3 seconds)

```bash
strataquant compare
```

Side-by-side comparison:
```
Strategy              Return %       Sharpe     Max DD %       Trades
========================================================================
Buy and Hold            857.31         0.99      -76.64            1
SMA Crossover             523.67         1.45      -48.23           12
SMA 20/50                 412.89         0.87      -52.11           28
SMA 100/200               634.12         1.23      -61.45            8
```

## Advanced Usage

### Custom Date Range

```bash
strataquant download --start 2020-01-01 --end 2023-12-31
```

### Different Time Intervals

```bash
# Hourly data (larger file, more data points)
strataquant download --interval 1h

# 5-minute data (very large file)
strataquant download --interval 5m
```

### Custom Parameters

```bash
# Different SMA periods
strataquant backtest --strategy sma --fast 20 --slow 50

# Different capital and costs
strataquant backtest --capital 50000 --commission 20 --slippage 10
```

### Fine-Grained Optimization

```bash
# Narrow range, small step
strataquant optimize \
    --fast-range 40-60 \
    --slow-range 150-250 \
    --step 5
```

### Strict Walk-Forward Split

```bash
# 80% train, 20% test
strataquant walkforward --train-ratio 0.8
```

## Understanding the Results

### Result Files Location

```
results/
├── backtests/
│   ├── buy_and_hold.json
│   ├── sma_50_200.json
│   └── ...
├── optimization_results.json
└── walkforward_result.json
```

### Reading JSON Results

Example `buy_and_hold.json`:
```json
{
  "initial_capital": 100000.0,
  "final_equity": 957307.75,
  "total_return": 8.5731,
  "equity_curve": [100000.0, 101234.5, ...],
  "total_trades": 1,
  "sharpe_ratio": 0.99,
  "max_drawdown": -0.7664
}
```

The `equity_curve` array contains your portfolio value at each time point.

## Interpreting Walk-Forward Results

**Good scenario:**
```
In-sample Sharpe: 1.5
Out-of-sample Sharpe: 1.2
Degradation: 20%
```
Strategy has some robustness.

**Bad scenario (typical):**
```
In-sample Sharpe: 2.1
Out-of-sample Sharpe: 0.4
Degradation: 81%
```
Strategy overfit to training data. Don't trust these parameters.

**Rule of thumb:**
- <30% degradation: Rare, worth investigating
- 30-50% degradation: Normal, acceptable
- 50-80% degradation: Expected, shows overfitting
- >80% degradation: Severe overfitting, parameters worthless

## Common Issues

### "Data file not found"

Solution: Run `strataquant download` first.

### "Rust not found"

Solution: Install Rust from https://rustup.rs/ and restart terminal.

### Slow optimization

Check CPU usage. Optimization uses all cores. On 4-core CPU, expect ~1 minute for 150 combinations.

### Different results than v0.1.0

v0.2.0 uses the Strategy trait with slightly different execution logic. Results should be within 1-2% of v0.1.0.

## Performance Benchmarks

**Hardware:** 8-core CPU, 16GB RAM, SSD

| Task | Time | Output Size |
|------|------|-------------|
| Download daily data | 30s | 40 MB |
| Single backtest | 1s | 2 KB |
| Optimize (150 combos) | 30s | 45 KB |
| Walk-forward | 45s | 1 KB |
| Compare strategies | 3s | - |

## Next Steps After Deployment

1. **Validate v0.1.0 results:** Run buy-and-hold, confirm 857% return
2. **Test SMA strategies:** Try different period combinations
3. **Run optimization:** See which parameters would have worked historically
4. **Critical step:** Run walk-forward validation
5. **Interpret degradation:** Understand what overfitting looks like

## Development Workflow

### Add New Strategy

1. Create file in `src/strategies/my_strategy.rs`
2. Implement `Strategy` trait
3. Add to `src/strategies/mod.rs`
4. Update CLI in `src/main.rs`
5. Run `cargo build --release`

### Test Changes

```bash
# Run clippy
cargo clippy

# Format code
cargo fmt

# Build
cargo build --release

# Test
cargo test
```

### Commit Changes

```bash
git add .
git commit -m "Add new strategy"
git push
```

## Production Deployment

### Server Setup

```bash
# Build on server
git clone https://github.com/ChronoCoders/strataquant.git
cd strataquant
cargo build --release

# Run as service
./target/release/strataquant download
./target/release/strataquant walkforward > daily_validation.log
```

### Automation

```bash
# Cron job for daily data updates
0 2 * * * cd /path/to/strataquant && ./target/release/strataquant download
```

### Monitoring

Check `results/` directory for new files. Parse JSON for automated alerts.

## Support

**Issues:** https://github.com/ChronoCoders/strataquant/issues
**Discussions:** GitHub Discussions
**Contact:** via GitHub profile

## Philosophy Reminder

StrataQuant prioritizes honesty:
- Shows real drawdowns
- Includes execution costs
- Exposes overfitting through walk-forward validation
- Doesn't cherry-pick parameters

Use this tool to understand what actually happened historically, not to predict what will happen next.

**Trading involves risk. Past performance does not guarantee future results.**
