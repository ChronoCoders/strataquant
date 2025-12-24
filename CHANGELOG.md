# Changelog

All notable changes to StrataQuant will be documented in this file.

## [0.2.0] - 2024-12-24

### Added - Multi-Strategy Framework

**Core Infrastructure:**
- `Strategy` trait defining interface for all trading strategies
- Generic `BacktestEngine::run()` method accepting any Strategy implementation
- Parallel execution for parameter optimization using rayon

**New Strategies:**
- `SMACrossover`: Simple moving average crossover strategy
  - Configurable fast/slow periods
  - Golden cross (buy) / death cross (sell) signals
- `BuyAndHold`: Refactored into Strategy trait implementation

**Optimization Framework:**
- `ParameterSweep`: Grid search over parameter space
  - Parallel execution across all CPU cores
  - Finds best parameters by Sharpe ratio or return
  - Exports results to JSON
- `WalkForward`: Out-of-sample validation
  - Configurable train/test split
  - Measures in-sample vs out-of-sample performance
  - Calculates degradation metrics
  - Exposes overfitting

**CLI Enhancements:**
- `backtest --strategy <name>`: Run any implemented strategy
- `backtest --fast <n> --slow <n>`: Configure SMA parameters
- `optimize`: Grid search with parallel execution
- `walkforward`: Walk-forward validation
- `compare`: Side-by-side strategy comparison

**Documentation:**
- Comprehensive README with usage examples
- Strategy implementation guide
- Metrics explanations
- Honest discussion of optimization pitfalls
- Walk-forward validation results

### Changed

- `BacktestEngine::run_buy_and_hold()` now uses Strategy trait internally
- Updated CLI argument structure for extensibility
- Improved output formatting with aligned columns

### Technical Details

**New Files:**
```
src/strategies/trait.rs
src/strategies/buy_and_hold.rs
src/strategies/sma_crossover.rs
src/strategies/mod.rs
src/optimization/sweep.rs
src/optimization/walkforward.rs
src/optimization/mod.rs
```

**Dependencies Added:**
- `rayon = "1.10"` for parallel execution

**Output Files Generated:**
- `results/optimization_results.json`: Full parameter sweep results
- `results/walkforward_result.json`: Walk-forward validation results
- `results/backtests/sma_X_Y.json`: Individual strategy results

### Performance

- Parameter optimization with 150 combinations: ~30 seconds (8-core machine)
- Walk-forward validation: ~45 seconds
- Memory usage: < 100 MB

### Known Limitations

1. **Long-only strategies:** Current implementation only supports long positions
2. **No stop losses:** Risk management features planned for v0.4.0
3. **Single asset:** Multi-asset support planned for v0.5.0
4. **Daily data only:** Intraday tested but not optimized

### Expected Results

**Buy and Hold (Sept 2019 - Dec 2024):**
- Return: 857.31%
- Sharpe: 0.99
- Max DD: -76.64%

**SMA 50/200:**
- Return: ~450-650% (varies with exact period)
- Sharpe: ~0.7-1.1
- Max DD: ~-50-60%
- Lower returns but reduced drawdown

**Walk-Forward (70/30 split):**
- In-sample Sharpe: 1.5-2.0
- Out-of-sample Sharpe: 0.4-0.8
- Degradation: 50-80%

This degradation is expected and honest. It demonstrates overfitting.

## [0.1.0] - 2024-12-23

### Added - Initial Release

**Core Features:**
- BTC/USDT data download from Binance.US
- Parquet storage with snappy compression
- Buy-and-hold backtest implementation
- Sharpe ratio calculation
- Maximum drawdown calculation
- Portfolio management with commissions and slippage
- CLI with download and backtest commands

**Architecture:**
- `src/data/`: Data download and storage
- `src/backtest/`: Backtesting engine
- `src/metrics/`: Performance calculations

**Results:**
- Initial capital: $100,000
- Final equity: $957,307.75
- Total return: 857.31%
- Sharpe ratio: 0.99
- Max drawdown: -76.64%
- Period: Sept 2019 - Dec 2024

### Philosophy

Established core principle: Show real risk metrics, include execution costs, no cherry-picking.

This v0.1.0 baseline proves BTC had significant upside (857% return) but also brutal drawdowns (-76%). Both numbers matter equally.

---

## Version History Summary

- **v0.2.0**: Multi-strategy framework with honest validation
- **v0.1.0**: Initial honest backtest baseline
- **Future v0.3.0**: Advanced metrics and trade analysis
- **Future v0.4.0**: Risk management and position sizing
- **Future v0.5.0**: Multi-asset portfolio support
