# StrataQuant v0.5.1 Release Notes

**Release Date:** December 26, 2025  
**Focus:** Chart Generation

---

## Overview

v0.5.1 adds chart generation capability to the backtest command via a simple `--plot` flag. Generate equity curves and drawdown charts as PNG files with zero configuration.

---

## What's New

### --plot Flag

```bash
strataquant backtest --strategy sma --fast 20 --slow 50 --plot
```

**Generates:**
- `results/charts/sma_20_50_equity.png`
- `results/charts/sma_20_50_drawdown.png`

### Chart Types

**Equity Curve:**
- Blue line showing portfolio value over time
- X-axis: Days
- Y-axis: Equity ($)
- 1200x600 resolution

**Drawdown Chart:**
- Red shaded area showing drawdown percentage
- X-axis: Days
- Y-axis: Drawdown (%)
- 1200x600 resolution

---

## Examples

### Basic Usage

```bash
# Run backtest with charts
cargo run --release --bin strataquant -- backtest --strategy sma --fast 20 --slow 50 --plot
```

**Output:**
```
=== RESULTS ===
Initial capital: $   100000.00
Final equity:    $   529247.21
Total return:         429.25%
Sharpe ratio:            0.81
Max drawdown:         -37.37%
Total trades:              18

Saved to: results/backtests\sma_20_50.json
Equity chart: results/charts\sma_20_50_equity.png
Drawdown chart: results/charts\sma_20_50_drawdown.png
```

### Without Charts

```bash
# Regular backtest (no charts)
cargo run --release --bin strataquant -- backtest --strategy sma --fast 20 --slow 50
```

Charts are only generated when `--plot` flag is present.

---

## Technical Details

### New Dependencies

```toml
[dependencies]
plotters = "0.3"  # Lightweight Rust plotting library
```

### New Module

```
src/plotting/mod.rs:
- plot_equity_curve()  # Generate equity chart
- plot_drawdown()      # Generate drawdown chart
```

### File Structure

```
results/
├── backtests/
│   └── sma_20_50.json
└── charts/
    ├── sma_20_50_equity.png
    └── sma_20_50_drawdown.png
```

---

## Performance

- Chart generation: ~100ms
- File size: 50-100 KB per PNG
- No impact on backtest when --plot not used
- Charts generated after backtest completes

---

## Code Quality

- Zero clippy warnings
- Proper error handling for file I/O
- Charts only generated if backtest succeeds

---

## Breaking Changes

None. `--plot` flag is completely optional.

---

## Migration Guide

### From v0.5.0

No changes required. All v0.5.0 functionality preserved.

```bash
# Update Cargo.toml
version = "0.5.1"

# Pull latest
git pull origin main
cargo build --release
```

---

## Known Limitations

1. **Backtest command only** - --plot not yet available for optimize/walkforward/compare
2. **PNG format only** - SVG/PDF export not yet implemented
3. **Fixed resolution** - 1200x600 hardcoded
4. **No customization** - Colors, fonts, sizes not configurable

---

## Future Enhancements (v0.6.0+)

Planned improvements:
- Add --plot to optimize/walkforward/compare commands
- SVG export option
- Configurable chart dimensions
- Custom colors and themes
- Multi-panel charts (equity + drawdown in one image)
- Interactive HTML charts

---

## Installation

```bash
# Clone repository
git clone https://github.com/ChronoCoders/strataquant.git
cd strataquant

# Checkout v0.5.1
git checkout v0.5.1

# Build
cargo build --release
```

---

## Testing

```bash
# Verify installation
cargo build --release

# Test chart generation
cargo run --release --bin strataquant -- backtest --strategy sma --fast 20 --slow 50 --plot

# Check output
ls results/charts/
```

---

## Credits

**Developer:** Altug Tatlisu (ChronoCoders)  
**Company:** Distributed Systems Labs, LLC  
**Repository:** https://github.com/ChronoCoders/strataquant  
**License:** MIT

---

## Support

- **Issues:** https://github.com/ChronoCoders/strataquant/issues
- **Discussions:** https://github.com/ChronoCoders/strataquant/discussions

---

**Full Changelog:** [CHANGELOG.md](CHANGELOG.md)  
**Previous Release:** [v0.5.0](https://github.com/ChronoCoders/strataquant/releases/tag/v0.5.0)  
**Repository:** [github.com/ChronoCoders/strataquant](https://github.com/ChronoCoders/strataquant)
