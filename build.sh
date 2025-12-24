#!/bin/bash
# StrataQuant v0.2.0 - Build and Test Script

set -e

echo "================================"
echo "StrataQuant v0.2.0 Build Script"
echo "================================"
echo ""

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Install from https://rustup.rs/"
    exit 1
fi

echo "✓ Rust found: $(rustc --version)"
echo ""

# Build in release mode
echo "Building release binary..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✓ Build successful"
else
    echo "❌ Build failed"
    exit 1
fi

echo ""

# Run clippy
echo "Running clippy..."
cargo clippy -- -D warnings

if [ $? -eq 0 ]; then
    echo "✓ Clippy passed (0 warnings)"
else
    echo "❌ Clippy found issues"
    exit 1
fi

echo ""

# Check binary
if [ -f "target/release/strataquant" ] || [ -f "target/release/strataquant.exe" ]; then
    echo "✓ Binary created successfully"
    
    if [ -f "target/release/strataquant" ]; then
        BINARY_SIZE=$(du -h target/release/strataquant | cut -f1)
        echo "  Size: $BINARY_SIZE"
    else
        BINARY_SIZE=$(du -h target/release/strataquant.exe | cut -f1)
        echo "  Size: $BINARY_SIZE"
    fi
else
    echo "❌ Binary not found"
    exit 1
fi

echo ""
echo "================================"
echo "Build Complete!"
echo "================================"
echo ""
echo "Next steps:"
echo "1. Download data:    ./target/release/strataquant download"
echo "2. Run backtest:     ./target/release/strataquant backtest"
echo "3. Test SMA:         ./target/release/strataquant backtest --strategy sma"
echo "4. Optimize:         ./target/release/strataquant optimize"
echo "5. Walk-forward:     ./target/release/strataquant walkforward"
echo "6. Compare all:      ./target/release/strataquant compare"
echo ""
