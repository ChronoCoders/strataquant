use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use std::path::Path;
use strataquant::backtest::{
    BacktestEngine, ExecutionModel, PositionSizingMethod, RiskLimits, StopLossMethod,
};
use strataquant::data::{load_from_parquet, save_to_parquet, BinanceDownloader};
use strataquant::optimization::{ParameterSweep, WalkForward};
use strataquant::strategies::{BuyAndHold, SMACrossover};

// Helper struct to group risk management parameters
struct RiskConfig {
    stop_type: String,
    stop_pct: f64,
    atr_multiplier: f64,
    atr_period: usize,
    time_limit: usize,
    position_sizing: String,
    position_size: f64,
    max_drawdown: f64,
}

#[derive(Parser)]
#[command(name = "strataquant")]
#[command(about = "StrataQuant - Honest BTC Backtesting Engine", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download historical BTC data from Binance
    Download {
        /// Start date (YYYY-MM-DD)
        #[arg(short, long, default_value = "2019-09-08")]
        start: String,

        /// End date (YYYY-MM-DD)
        #[arg(short, long, default_value = "2024-12-22")]
        end: String,

        /// Interval (1d, 1h, 5m, etc)
        #[arg(short, long, default_value = "1d")]
        interval: String,
    },

    /// Run backtest on downloaded data
    Backtest {
        /// Strategy to use (buy-and-hold, sma)
        #[arg(short = 't', long, default_value = "buy-and-hold")]
        strategy: String,

        /// Fast SMA period (for SMA strategy)
        #[arg(short = 'f', long, default_value = "50")]
        fast: usize,

        /// Slow SMA period (for SMA strategy)
        #[arg(short = 'w', long, default_value = "200")]
        slow: usize,

        /// Initial capital in USD
        #[arg(short, long, default_value = "100000")]
        capital: f64,

        /// Commission in basis points
        #[arg(short = 'm', long, default_value = "10")]
        commission: f64,

        /// Slippage in basis points
        #[arg(short = 'l', long, default_value = "5")]
        slippage: f64,

        /// Stop loss type: none, fixed, trailing, atr, time
        #[arg(long, default_value = "none")]
        stop_type: String,

        /// Stop loss percentage (for fixed/trailing)
        #[arg(long, default_value = "10.0")]
        stop_pct: f64,

        /// ATR multiplier (for atr stop type)
        #[arg(long, default_value = "2.0")]
        atr_multiplier: f64,

        /// ATR period (for atr stop type)
        #[arg(long, default_value = "14")]
        atr_period: usize,

        /// Time limit in bars (for time stop type)
        #[arg(long, default_value = "100")]
        time_limit: usize,

        /// Position sizing method: fixed-pct, fixed-dollar, kelly, half-kelly
        #[arg(long, default_value = "fixed-pct")]
        position_sizing: String,

        /// Position size (percentage or dollar amount depending on method)
        #[arg(long, default_value = "100.0")]
        position_size: f64,

        /// Maximum drawdown threshold (stop trading at -X%)
        #[arg(long, default_value = "50.0")]
        max_drawdown: f64,
    },

    /// Optimize SMA parameters with grid search
    Optimize {
        /// Fast period range (min-max)
        #[arg(long, default_value = "20-100")]
        fast_range: String,

        /// Slow period range (min-max)
        #[arg(long, default_value = "50-200")]
        slow_range: String,

        /// Step size for parameter sweep
        #[arg(long, default_value = "10")]
        step: usize,

        /// Initial capital in USD
        #[arg(short, long, default_value = "100000")]
        capital: f64,

        /// Commission in basis points
        #[arg(short = 'm', long, default_value = "10")]
        commission: f64,

        /// Slippage in basis points
        #[arg(short = 'l', long, default_value = "5")]
        slippage: f64,

        /// Stop loss type: none, fixed, trailing, atr
        #[arg(long, default_value = "none")]
        stop_type: String,

        /// Stop loss percentage (for fixed/trailing)
        #[arg(long, default_value = "10.0")]
        stop_pct: f64,

        /// ATR multiplier (for atr stop type)
        #[arg(long, default_value = "2.0")]
        atr_multiplier: f64,

        /// ATR period (for atr stop type)
        #[arg(long, default_value = "14")]
        atr_period: usize,
    },

    /// Walk-forward validation
    Walkforward {
        /// Train/test split ratio (0.0-1.0)
        #[arg(long, default_value = "0.7")]
        train_ratio: f64,

        /// Initial capital in USD
        #[arg(short, long, default_value = "100000")]
        capital: f64,

        /// Commission in basis points
        #[arg(short = 'm', long, default_value = "10")]
        commission: f64,

        /// Slippage in basis points
        #[arg(short = 'l', long, default_value = "5")]
        slippage: f64,

        /// Stop loss type: none, fixed, trailing, atr
        #[arg(long, default_value = "none")]
        stop_type: String,

        /// Stop loss percentage (for fixed/trailing)
        #[arg(long, default_value = "10.0")]
        stop_pct: f64,

        /// ATR multiplier (for atr stop type)
        #[arg(long, default_value = "2.0")]
        atr_multiplier: f64,

        /// ATR period (for atr stop type)
        #[arg(long, default_value = "14")]
        atr_period: usize,
    },

    /// Compare all strategies
    Compare {
        /// Initial capital in USD
        #[arg(short, long, default_value = "100000")]
        capital: f64,

        /// Commission in basis points
        #[arg(short = 'm', long, default_value = "10")]
        commission: f64,

        /// Slippage in basis points
        #[arg(short = 'l', long, default_value = "5")]
        slippage: f64,

        /// Stop loss type: none, fixed, trailing, atr
        #[arg(long, default_value = "none")]
        stop_type: String,

        /// Stop loss percentage (for fixed/trailing)
        #[arg(long, default_value = "10.0")]
        stop_pct: f64,

        /// ATR multiplier (for atr stop type)
        #[arg(long, default_value = "2.0")]
        atr_multiplier: f64,

        /// ATR period (for atr stop type)
        #[arg(long, default_value = "14")]
        atr_period: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Download {
            start,
            end,
            interval,
        } => {
            download_data(&start, &end, &interval);
        }
        Commands::Backtest {
            strategy,
            fast,
            slow,
            capital,
            commission,
            slippage,
            stop_type,
            stop_pct,
            atr_multiplier,
            atr_period,
            time_limit,
            position_sizing,
            position_size,
            max_drawdown,
        } => {
            run_backtest(
                &strategy,
                fast,
                slow,
                capital,
                commission,
                slippage,
                RiskConfig {
                    stop_type,
                    stop_pct,
                    atr_multiplier,
                    atr_period,
                    time_limit,
                    position_sizing,
                    position_size,
                    max_drawdown,
                },
            );
        }
        Commands::Optimize {
            fast_range,
            slow_range,
            step,
            capital,
            commission,
            slippage,
            stop_type,
            stop_pct,
            atr_multiplier,
            atr_period,
        } => {
            let execution_model = ExecutionModel::new(commission, slippage);
            let stop_loss = parse_stop_loss(&stop_type, stop_pct, atr_multiplier, atr_period);
            run_optimization(
                &fast_range,
                &slow_range,
                step,
                capital,
                execution_model,
                stop_loss,
            );
        }
        Commands::Walkforward {
            train_ratio,
            capital,
            commission,
            slippage,
            stop_type,
            stop_pct,
            atr_multiplier,
            atr_period,
        } => {
            let execution_model = ExecutionModel::new(commission, slippage);
            let stop_loss = parse_stop_loss(&stop_type, stop_pct, atr_multiplier, atr_period);
            run_walkforward(train_ratio, capital, execution_model, stop_loss);
        }
        Commands::Compare {
            capital,
            commission,
            slippage,
            stop_type,
            stop_pct,
            atr_multiplier,
            atr_period,
        } => {
            let execution_model = ExecutionModel::new(commission, slippage);
            let stop_loss = parse_stop_loss(&stop_type, stop_pct, atr_multiplier, atr_period);
            run_comparison(capital, execution_model, stop_loss);
        }
    }
}

fn download_data(start: &str, end: &str, interval: &str) {
    println!("StrataQuant - Data Download");
    println!("===========================\n");

    let downloader = BinanceDownloader::new("BTCUSDT", interval);

    let start_dt = DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", start))
        .expect("Invalid start date")
        .with_timezone(&Utc);

    let end_dt = DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", end))
        .expect("Invalid end date")
        .with_timezone(&Utc);

    println!("Downloading BTC/USDT {} data", interval);
    println!("From: {}", start_dt);
    println!("To:   {}\n", end_dt);

    match downloader.fetch_range(start_dt, end_dt) {
        Ok(data) => {
            println!("\nDownloaded {} candles", data.len());

            let filename = format!("data/processed/btc_{}.parquet", interval);
            let output_path = Path::new(&filename);
            println!("Saving to: {}", output_path.display());

            match save_to_parquet(&data, output_path) {
                Ok(_) => {
                    let file_size = std::fs::metadata(output_path).unwrap().len();
                    println!(
                        "Success! File size: {:.2} MB",
                        file_size as f64 / 1_024_000.0
                    );
                }
                Err(e) => {
                    eprintln!("Failed to save: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Download failed: {}", e);
            std::process::exit(1);
        }
    }
}

// Helper function to parse stop loss method from CLI args
fn parse_stop_loss(
    stop_type: &str,
    stop_pct: f64,
    atr_multiplier: f64,
    atr_period: usize,
) -> StopLossMethod {
    match stop_type {
        "none" => StopLossMethod::None,
        "fixed" => StopLossMethod::FixedPercent(stop_pct),
        "trailing" => StopLossMethod::Trailing(stop_pct),
        "atr" => StopLossMethod::ATR {
            multiplier: atr_multiplier,
            period: atr_period,
        },
        _ => {
            eprintln!("Unknown stop type: {}", stop_type);
            eprintln!("Available: none, fixed, trailing, atr");
            std::process::exit(1);
        }
    }
}

fn run_backtest(
    strategy_name: &str,
    fast: usize,
    slow: usize,
    capital: f64,
    commission: f64,
    slippage: f64,
    risk_config: RiskConfig,
) {
    println!("StrataQuant - Backtest");
    println!("======================\n");

    let data_path = Path::new("data/processed/btc_1d.parquet");

    if !data_path.exists() {
        eprintln!("Error: Data file not found at {}", data_path.display());
        eprintln!("Run 'strataquant download' first");
        std::process::exit(1);
    }

    println!("Loading data from: {}", data_path.display());
    let data = match load_from_parquet(data_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to load data: {}", e);
            std::process::exit(1);
        }
    };

    println!("Loaded {} candles", data.len());
    println!(
        "Period: {} to {}\n",
        chrono::DateTime::from_timestamp_millis(data[0].timestamp).unwrap(),
        chrono::DateTime::from_timestamp_millis(data[data.len() - 1].timestamp).unwrap()
    );

    let execution_model = ExecutionModel::new(commission, slippage);

    let (strategy_display, output_filename): (Box<dyn strataquant::strategies::Strategy>, String) =
        match strategy_name {
            "buy-and-hold" => (Box::new(BuyAndHold::new()), "buy_and_hold.json".to_string()),
            "sma" => (
                Box::new(SMACrossover::new(fast, slow)),
                format!("sma_{}_{}.json", fast, slow),
            ),
            _ => {
                eprintln!("Unknown strategy: {}", strategy_name);
                eprintln!("Available strategies: buy-and-hold, sma");
                std::process::exit(1);
            }
        };

    println!("Strategy: {}", strategy_display.name());
    println!("Description: {}", strategy_display.description());
    println!("Initial capital: ${:.2}", capital);
    println!("Commission: {} bps", commission);
    println!("Slippage: {} bps", slippage);

    // Parse stop loss method (including time limit for backtest)
    let stop_loss = match risk_config.stop_type.as_str() {
        "none" => StopLossMethod::None,
        "fixed" => StopLossMethod::FixedPercent(risk_config.stop_pct),
        "trailing" => StopLossMethod::Trailing(risk_config.stop_pct),
        "atr" => StopLossMethod::ATR {
            multiplier: risk_config.atr_multiplier,
            period: risk_config.atr_period,
        },
        "time" => StopLossMethod::TimeLimit(risk_config.time_limit),
        _ => {
            eprintln!("Unknown stop type: {}", risk_config.stop_type);
            eprintln!("Available: none, fixed, trailing, atr, time");
            std::process::exit(1);
        }
    };

    // Parse position sizing method
    let position_sizing = match risk_config.position_sizing.as_str() {
        "fixed-pct" => PositionSizingMethod::FixedPercent(risk_config.position_size),
        "fixed-dollar" => PositionSizingMethod::FixedDollar(risk_config.position_size),
        _ => {
            eprintln!("Unknown position sizing: {}", risk_config.position_sizing);
            eprintln!("Available: fixed-pct, fixed-dollar");
            std::process::exit(1);
        }
    };

    // Create risk limits only if user changed defaults
    let risk_limits = if risk_config.max_drawdown < 50.0 {
        Some(RiskLimits {
            max_drawdown_threshold: risk_config.max_drawdown / 100.0,
            ..Default::default()
        })
    } else {
        None
    };

    // Display risk parameters
    match &stop_loss {
        StopLossMethod::None => println!("Stop loss: None"),
        StopLossMethod::FixedPercent(pct) => println!("Stop loss: Fixed {:.1}%", pct),
        StopLossMethod::Trailing(pct) => println!("Stop loss: Trailing {:.1}%", pct),
        StopLossMethod::ATR { multiplier, period } => {
            println!("Stop loss: ATR ({} x {} period)", multiplier, period)
        }
        StopLossMethod::TimeLimit(bars) => println!("Stop loss: Time limit {} bars", bars),
    }

    match &position_sizing {
        PositionSizingMethod::FixedPercent(pct) => {
            println!("Position size: {:.1}% of equity", pct)
        }
        PositionSizingMethod::FixedDollar(amt) => println!("Position size: ${:.0}", amt),
        _ => {}
    }

    println!("Max drawdown threshold: {:.1}%\n", risk_config.max_drawdown);

    let mut engine = BacktestEngine::new(data, capital, execution_model)
        .with_stop_loss(stop_loss)
        .with_position_sizing(position_sizing);

    if let Some(limits) = risk_limits {
        engine = engine.with_risk_limits(limits);
    }

    println!("Running backtest...\n");
    let result = engine.run(strategy_display.as_ref());

    println!("=== RESULTS ===");
    println!("Initial capital: ${:>12.2}", result.initial_capital);
    println!("Final equity:    ${:>12.2}", result.final_equity);
    println!("Total return:    {:>11.2}%", result.total_return * 100.0);
    println!("Sharpe ratio:    {:>12.2}", result.sharpe_ratio);
    println!("Sortino ratio:   {:>12.2}", result.sortino_ratio);
    println!("Calmar ratio:    {:>12.2}", result.calmar_ratio);
    println!("Max drawdown:    {:>11.2}%", result.max_drawdown * 100.0);
    println!("Total trades:    {:>12}", result.total_trades);

    if let Some(stats) = &result.trade_stats {
        println!("\n=== TRADE ANALYSIS ===");
        println!("Win rate:        {:>11.1}%", stats.win_rate * 100.0);
        println!("Profit factor:   {:>12.2}", stats.profit_factor);
        println!("Avg win:         ${:>11.2}", stats.avg_win);
        println!("Avg loss:        ${:>11.2}", stats.avg_loss);
        println!("Largest win:     ${:>11.2}", stats.largest_win);
        println!("Largest loss:    ${:>11.2}", stats.largest_loss);
        println!("Expectancy:      ${:>11.2}", stats.expectancy);
        println!("Win streak:      {:>12}", stats.longest_win_streak);
        println!("Loss streak:     {:>12}", stats.longest_loss_streak);
    }

    let output_path = Path::new("results/backtests").join(output_filename);
    match result.save_to_file(&output_path) {
        Ok(_) => println!("\nSaved to: {}", output_path.display()),
        Err(e) => eprintln!("Failed to save: {}", e),
    }
}

fn parse_range(range: &str) -> (usize, usize) {
    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() != 2 {
        eprintln!("Invalid range format: {}", range);
        eprintln!("Expected format: min-max (e.g., 20-100)");
        std::process::exit(1);
    }

    let min = parts[0].parse().expect("Invalid min value");
    let max = parts[1].parse().expect("Invalid max value");

    (min, max)
}

fn run_optimization(
    fast_range: &str,
    slow_range: &str,
    step: usize,
    capital: f64,
    execution_model: ExecutionModel,
    stop_loss: StopLossMethod,
) {
    println!("StrataQuant - Parameter Optimization");
    println!("====================================\n");

    let data_path = Path::new("data/processed/btc_1d.parquet");
    if !data_path.exists() {
        eprintln!("Error: Data file not found");
        std::process::exit(1);
    }

    let data = load_from_parquet(data_path).expect("Failed to load data");
    println!("Loaded {} candles\n", data.len());

    let (fast_min, fast_max) = parse_range(fast_range);
    let (slow_min, slow_max) = parse_range(slow_range);

    println!("Fast period range: {}-{}", fast_min, fast_max);
    println!("Slow period range: {}-{}", slow_min, slow_max);
    println!("Step size: {}", step);

    match &stop_loss {
        StopLossMethod::None => println!("Stop loss: None"),
        StopLossMethod::FixedPercent(pct) => println!("Stop loss: Fixed {:.1}%", pct),
        StopLossMethod::Trailing(pct) => println!("Stop loss: Trailing {:.1}%", pct),
        StopLossMethod::ATR { multiplier, period } => {
            println!("Stop loss: ATR ({} x {} period)", multiplier, period)
        }
        _ => {}
    }
    println!();

    // Create sweep with stop loss
    let sweep = ParameterSweep::new(data, capital, execution_model);

    // Run optimization with stop loss
    println!(
        "Testing {} combinations...",
        ((fast_max - fast_min) / step + 1) * ((slow_max - slow_min) / step + 1)
    );

    let results = sweep.sweep_sma_periods_with_stops(
        (fast_min, fast_max),
        (slow_min, slow_max),
        step,
        stop_loss,
    );

    println!(
        "\nOptimization complete. Tested {} combinations.\n",
        results.len()
    );

    let best_sharpe = ParameterSweep::find_best_sharpe(&results).unwrap();
    let best_return = ParameterSweep::find_best_return(&results).unwrap();

    println!("=== BEST BY SHARPE RATIO ===");
    println!(
        "Parameters: {}/{}",
        best_sharpe.fast_period, best_sharpe.slow_period
    );
    println!("Sharpe ratio: {:.2}", best_sharpe.sharpe_ratio);
    println!("Total return: {:.2}%", best_sharpe.total_return * 100.0);
    println!("Max drawdown: {:.2}%", best_sharpe.max_drawdown * 100.0);
    println!("Total trades: {}\n", best_sharpe.total_trades);

    println!("=== BEST BY TOTAL RETURN ===");
    println!(
        "Parameters: {}/{}",
        best_return.fast_period, best_return.slow_period
    );
    println!("Total return: {:.2}%", best_return.total_return * 100.0);
    println!("Sharpe ratio: {:.2}", best_return.sharpe_ratio);
    println!("Max drawdown: {:.2}%", best_return.max_drawdown * 100.0);
    println!("Total trades: {}", best_return.total_trades);

    let output_path = Path::new("results/optimization_results.json");
    ParameterSweep::save_results(&results, output_path).expect("Failed to save results");
    println!("\nFull results saved to: {}", output_path.display());
}

fn run_walkforward(
    train_ratio: f64,
    capital: f64,
    execution_model: ExecutionModel,
    stop_loss: StopLossMethod,
) {
    println!("StrataQuant - Walk-Forward Validation");
    println!("=====================================\n");

    let data_path = Path::new("data/processed/btc_1d.parquet");
    if !data_path.exists() {
        eprintln!("Error: Data file not found");
        std::process::exit(1);
    }

    let data = load_from_parquet(data_path).expect("Failed to load data");

    match &stop_loss {
        StopLossMethod::None => println!("Stop loss: None"),
        StopLossMethod::FixedPercent(pct) => println!("Stop loss: Fixed {:.1}%", pct),
        StopLossMethod::Trailing(pct) => println!("Stop loss: Trailing {:.1}%", pct),
        StopLossMethod::ATR { multiplier, period } => {
            println!("Stop loss: ATR ({} x {} period)", multiplier, period)
        }
        _ => {}
    }
    println!();

    let walkforward = WalkForward::new(data, capital, execution_model);

    let result = walkforward.run_with_stops(train_ratio, stop_loss);

    println!("=== WALK-FORWARD RESULTS ===");
    println!(
        "Optimal parameters: {}/{}",
        result.best_fast_period, result.best_slow_period
    );
    println!("\nIn-Sample (Training):");
    println!("  Return:       {:>8.2}%", result.in_sample_return * 100.0);
    println!("  Sharpe ratio: {:>8.2}", result.in_sample_sharpe);
    println!("\nOut-of-Sample (Testing):");
    println!(
        "  Return:       {:>8.2}%",
        result.out_of_sample_return * 100.0
    );
    println!("  Sharpe ratio: {:>8.2}", result.out_of_sample_sharpe);
    println!("\nPerformance Degradation:");
    println!("  Return:       {:>8.2}%", result.degradation_return);
    println!("  Sharpe ratio: {:>8.2}%", result.degradation_sharpe);

    if result.degradation_sharpe > 50.0 {
        println!("\n⚠️  WARNING: Significant degradation detected!");
        println!("This suggests the optimization may have overfit to the training data.");
    }

    let output_path = Path::new("results/walkforward_result.json");
    WalkForward::save_result(&result, output_path).expect("Failed to save result");
    println!("\nResults saved to: {}", output_path.display());
}

fn run_comparison(capital: f64, execution_model: ExecutionModel, stop_loss: StopLossMethod) {
    println!("StrataQuant - Strategy Comparison");
    println!("=================================\n");

    let data_path = Path::new("data/processed/btc_1d.parquet");
    if !data_path.exists() {
        eprintln!("Error: Data file not found");
        std::process::exit(1);
    }

    let data = load_from_parquet(data_path).expect("Failed to load data");
    println!("Loaded {} candles", data.len());

    match &stop_loss {
        StopLossMethod::None => println!("Stop loss: None"),
        StopLossMethod::FixedPercent(pct) => println!("Stop loss: Fixed {:.1}%", pct),
        StopLossMethod::Trailing(pct) => println!("Stop loss: Trailing {:.1}%", pct),
        StopLossMethod::ATR { multiplier, period } => {
            println!("Stop loss: ATR ({} x {} period)", multiplier, period)
        }
        _ => {}
    }
    println!();

    let strategies: Vec<Box<dyn strataquant::strategies::Strategy>> = vec![
        Box::new(BuyAndHold::new()),
        Box::new(SMACrossover::new(50, 200)),
        Box::new(SMACrossover::new(20, 50)),
        Box::new(SMACrossover::new(100, 200)),
    ];

    println!(
        "{:<20} {:>12} {:>12} {:>12} {:>12}",
        "Strategy", "Return %", "Sharpe", "Max DD %", "Trades"
    );
    println!("{}", "=".repeat(72));

    for strategy in strategies {
        let engine = BacktestEngine::new(data.clone(), capital, execution_model.clone())
            .with_stop_loss(stop_loss.clone());
        let result = engine.run(strategy.as_ref());

        println!(
            "{:<20} {:>11.2}% {:>12.2} {:>11.2}% {:>12}",
            strategy.name(),
            result.total_return * 100.0,
            result.sharpe_ratio,
            result.max_drawdown * 100.0,
            result.total_trades
        );
    }

    println!("\nComparison complete.");
}
