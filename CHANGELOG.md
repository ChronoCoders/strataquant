# Changelog

All notable changes to StrataQuant will be documented in this file.

## [0.5.1] - 2025-12-26

### Added - Chart Generation

**--plot Flag:**
- Added `--plot` flag to backtest command
- Generates equity curve chart (PNG)
- Generates drawdown chart (PNG)
- Charts saved to `results/charts/` directory

**Usage:**
```bash
strataquant backtest --strategy sma --fast 20 --slow 50 --plot
```

**Output Files:**
- `results/charts/sma_20_50_equity.png` - Blue line showing portfolio value over time
- `results/charts/sma_20_50_drawdown.png` - Red area showing drawdown percentage

### Technical Details

**New Files:**
```
src/plotting/mod.rs         # Plotting module
```

**Modified Files:**
```
src/lib.rs                  # Export plotting module
src/main.rs                 # Add --plot flag
Cargo.toml                  # Add plotters dependency
```

**Dependencies:**
- `plotters = "0.3"` - Rust plotting library

**Chart Specifications:**
- Resolution: 1200x600 pixels
- Equity chart: Blue line on white background
- Drawdown chart: Red shaded area
- File format: PNG
- Size: 50-100 KB per chart

### Performance

- Chart generation: ~100ms
- No impact when --plot not used
- Zero clippy warnings

## [0.5.0] - 2024-12-26

### Added - CLI Integration for Risk Management

Full CLI integration for all risk management features.

## [0.4.0] - 2024-12-25

### Added - Risk Management System

Stop losses, position sizing, risk limits.

## [0.3.0] - 2024-12-24

### Added - Advanced Metrics & Trade Analysis

Sortino ratio, Calmar ratio, trade tracking.

## [0.2.0] - 2024-12-24

### Added - Multi-Strategy Framework

Strategy trait, optimization, walk-forward validation.

## [0.1.0] - 2024-12-23

### Added - Initial Release

Core backtesting engine with honest metrics.

---

## Version History Summary

- **v0.5.1**: Chart generation with --plot flag
- **v0.5.0**: CLI integration for risk management
- **v0.4.0**: Risk management system
- **v0.3.0**: Advanced metrics and trade analysis
- **v0.2.0**: Multi-strategy framework
- **v0.1.0**: Initial release
- **Future v0.6.0**: Kelly, volatility sizing, multi-timeframe
- **Future v0.7.0**: Multi-asset (BTC, ETH, LTC, BCH, SOL)
