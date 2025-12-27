# StrataQuant v0.5.1

Truth in crypto backtesting. Production-grade backtesting engine with chart generation.

## What This Is

StrataQuant is a quantitative research engine for Bitcoin backtesting that prioritizes honesty over presentation. This tool shows real risk metrics, includes execution costs, and doesn't cherry-pick results.

**v0.5.1 adds:**
- Chart generation with --plot flag
- Equity curve charts (PNG)
- Drawdown charts (PNG)

**v0.5.0 added:**
- Full CLI integration for risk management
- Stop losses for all commands
- Position sizing controls
- Risk limit configuration

**v0.4.0 added:**
- Stop loss methods (fixed, trailing, ATR, time-based)
- Position sizing (fixed percent, fixed dollar)
- Risk limits (max drawdown, position size controls)

**v0.3.0 added:**
- Sortino ratio (downside deviation only)
- Calmar ratio (return per unit of drawdown)
- Complete trade tracking and analysis

## Philosophy

Most backtesting engines show you inflated returns and hide the drawdowns. StrataQuant does the opposite:

- Shows real drawdowns (-76.64% for buy-and-hold)
- Includes execution costs (15 bps total)
- No parameter cherry-picking
- Walk-forward validation exposes overfitting
- Complete trade analysis
- Advanced risk metrics
- Visual charts for analysis

## Installation

```bash
git clone https://github.com/ChronoCoders/strataquant.git
cd strataquant
cargo build --release
```

## Quick Start

```bash
# 1. Download data
strataquant download --start 2019-09-08 --end 2025-12-25

# 2. Run backtest with charts
strataquant backtest --strategy sma --fast 20 --slow 50 --plot

# 3. Optimize parameters
strataquant optimize --fast-range 20-50 --slow-range 50-100

# 4. Walk-forward validation
strataquant walkforward --train-ratio 0.7

# 5. Compare strategies
strataquant compare
```

## Commands

### backtest

```bash
strataquant backtest [OPTIONS]

Options:
  -t, --strategy <n>       Strategy: buy-and-hold, sma (default: buy-and-hold)
  -f, --fast <n>           Fast SMA period (default: 50)
  -w, --slow <n>           Slow SMA period (default: 200)
  -c, --capital <usd>      Initial capital (default: 100000)
  -m, --commission <bps>   Commission (default: 10)
  -l, --slippage <bps>     Slippage (default: 5)
  --plot                   Generate charts (NEW in v0.5.1)
```

**Examples:**

```bash
# Basic backtest
strataquant backtest --strategy sma --fast 20 --slow 50

# With charts
strataquant backtest --strategy sma --fast 20 --slow 50 --plot
```

**Chart Output:**
- `results/charts/sma_20_50_equity.png`
- `results/charts/sma_20_50_drawdown.png`

### optimize

Grid search over parameter space.

```bash
strataquant optimize --fast-range 20-50 --slow-range 50-100
```

### walkforward

Out-of-sample validation.

```bash
strataquant walkforward --train-ratio 0.7
```

### compare

Compare all strategies side-by-side.

```bash
strataquant compare
```

## Results (2019-2025)

### SMA 20/50

```
Total return:         429.25%
Sharpe ratio:            0.81
Max drawdown:         -37.37%
Total trades:              18
```

## Chart Examples

**Equity Curve:**
- Blue line showing portfolio growth
- 1200x600 resolution
- PNG format

**Drawdown:**
- Red shaded area
- Shows peak-to-trough declines
- 1200x600 resolution

## Roadmap

**✓ v0.5.1**: Chart generation  
**✓ v0.5.0**: CLI risk management  
**v0.6.0**: Kelly criterion, volatility sizing, multi-timeframe  
**v0.7.0**: Multi-asset (BTC, ETH, LTC, BCH, SOL)

## Development

```bash
cargo test
cargo clippy
cargo fmt
```

## License

MIT

## Contact

Built by Distributed Systems Labs, LLC  
GitHub: [@ChronoCoders](https://github.com/ChronoCoders)  
Website: https://dslabs.network

## Disclaimer

For educational and research purposes only. Past performance does not guarantee future results. Trading cryptocurrencies carries significant risk.

StrataQuant shows you honest metrics so you can make informed decisions.

## Version History

- **v0.5.1** - Chart generation with --plot flag
- **v0.5.0** - CLI integration for risk management
- **v0.4.0** - Risk management system
- **v0.3.0** - Advanced metrics and trade analysis
- **v0.2.0** - Multi-strategy framework
- **v0.1.0** - Initial release
