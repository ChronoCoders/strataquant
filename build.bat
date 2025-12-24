@echo off
REM StrataQuant v0.2.0 - Build and Test Script (Windows)

echo ================================
echo StrataQuant v0.2.0 Build Script
echo ================================
echo.

REM Check Rust installation
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo X Rust not found. Install from https://rustup.rs/
    exit /b 1
)

for /f "tokens=*" %%i in ('rustc --version') do set RUST_VERSION=%%i
echo + Rust found: %RUST_VERSION%
echo.

REM Build in release mode
echo Building release binary...
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo X Build failed
    exit /b 1
)

echo + Build successful
echo.

REM Run clippy
echo Running clippy...
cargo clippy -- -D warnings

if %ERRORLEVEL% NEQ 0 (
    echo X Clippy found issues
    exit /b 1
)

echo + Clippy passed (0 warnings)
echo.

REM Check binary
if exist target\release\strataquant.exe (
    echo + Binary created successfully
    for %%A in (target\release\strataquant.exe) do echo   Size: %%~zA bytes
) else (
    echo X Binary not found
    exit /b 1
)

echo.
echo ================================
echo Build Complete!
echo ================================
echo.
echo Next steps:
echo 1. Download data:    target\release\strataquant.exe download
echo 2. Run backtest:     target\release\strataquant.exe backtest
echo 3. Test SMA:         target\release\strataquant.exe backtest --strategy sma
echo 4. Optimize:         target\release\strataquant.exe optimize
echo 5. Walk-forward:     target\release\strataquant.exe walkforward
echo 6. Compare all:      target\release\strataquant.exe compare
echo.
