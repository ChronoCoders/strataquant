# StrataQuant v0.1.0

Honest BTC backtesting engine with real risk metrics. No illusions, just infrastructure and iteration.

## Features

- Download historical BTC/USDT data from Binance.US
- Buy-and-hold backtest with realistic execution costs
- Honest metrics: Sharpe ratio, max drawdown, total return
- Parquet storage for fast data access
- Command-line interface

## Installation

Requires Rust 1.70+
```cmd
git clone <repository>
cd strataquant
cargo build --release
```

## Usage

### 1. Download Data

Download BTC daily data from 2019 to present:
```cmd
target\release\strataquant.exe download
```

Custom date range:
```cmd
target\release\strataquant.exe download --start 2020-01-01 --end 2024-12-22
```

### 2. Run Backtest

Run buy-and-hold backtest with default parameters:
```cmd
target\release\strataquant.exe backtest
```

Custom parameters:
```cmd
target\release\strataquant.exe backtest --capital 50000 --commission 15 --slippage 10
```

## Example Output
```
StrataQuant - Backtest
======================

Loading data from: data/processed/btc_1d.parquet
Loaded 1919 candles
Period: 2019-09-23 00:00:00 UTC to 2024-12-22 00:00:00 UTC

Strategy: Buy and Hold
Initial capital: $100000.00
Commission: 10 bps
Slippage: 5 bps

Running backtest...

=== RESULTS ===
Initial capital: $   100000.00
Final equity:    $   957307.75
Total return:         857.31%
Sharpe ratio:            0.99
Max drawdown:         -76.64%
Total trades:               1

Saved to: results/backtests/buy_and_hold.json
```

## Architecture
```
strataquant/
├── src/
│   ├── data/           # Data download and storage
│   ├── backtest/       # Backtesting engine
│   ├── metrics/        # Risk calculations
│   └── main.rs         # CLI interface
├── data/
│   └── processed/      # Parquet files
└── results/
    └── backtests/      # JSON output
```

## Data Source

BTC/USDT data from Binance.US API:
- History: September 2019 - Present
- Interval: Daily (1d)
- No authentication required
- Free tier: 500 requests/minute

## Metrics

**Sharpe Ratio**: Annualized risk-adjusted return
- Formula: `(mean_return / std_return) * sqrt(365)`
- Interpretation: >1.0 is good, >2.0 is excellent

**Max Drawdown**: Peak-to-trough decline
- Formula: `(equity - running_max) / running_max`
- Shows worst case loss from peak

**Total Return**: Percentage gain/loss
- Formula: `(final_equity - initial_capital) / initial_capital`

## Execution Model

Realistic trading costs are included:

- **Commission**: 10 bps (0.10%) default
- **Slippage**: 5 bps (0.05%) default
- Market orders only (no limit orders)
- Instant execution at close price + slippage

## Limitations (v0.1.0)

- Single strategy only (buy-and-hold)
- Daily data only (no intraday)
- BTC/USDT only (no other assets)
- No parameter optimization
- No walk-forward validation

## Roadmap

**v0.2.0** (Planned):
- Python bindings via PyO3
- SMA crossover strategy
- Hourly data support
- Parameter sweeps

**v0.3.0** (Planned):
- Multiple assets
- Portfolio backtesting
- Advanced metrics (Sortino, Calmar)

## License

MIT

## Philosophy

StrataQuant's purpose is **truth in crypto backtesting**.

Most trading software lies about returns. This engine doesn't promise profits — it delivers clarity.

Clarity = informed strategy design = potential alpha in real markets.