use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use strataquant::backtest::{BacktestEngine, ExecutionModel};
use strataquant::data::{load_from_parquet, save_to_parquet, BinanceDownloader};

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
        /// Initial capital in USD
        #[arg(short, long, default_value = "100000")]
        capital: f64,

        /// Commission in basis points
        #[arg(short = 'm', long, default_value = "10")]
        commission: f64,

        /// Slippage in basis points
        #[arg(short, long, default_value = "5")]
        slippage: f64,
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
            capital,
            commission,
            slippage,
        } => {
            run_backtest(capital, commission, slippage);
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
            let output_path = PathBuf::from(&filename);

            println!("Saving to: {}", output_path.display());

            match save_to_parquet(&data, &output_path) {
                Ok(_) => {
                    let file_size = std::fs::metadata(&output_path).unwrap().len();
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

fn run_backtest(capital: f64, commission: f64, slippage: f64) {
    println!("StrataQuant - Backtest");
    println!("======================\n");

    let data_path = PathBuf::from("data/processed/btc_1d.parquet");

    if !data_path.exists() {
        eprintln!("Error: Data file not found at {}", data_path.display());
        eprintln!("Run 'strataquant download' first");
        std::process::exit(1);
    }

    println!("Loading data from: {}", data_path.display());
    let data = match load_from_parquet(&data_path) {
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

    println!("Strategy: Buy and Hold");
    println!("Initial capital: ${:.2}", capital);
    println!("Commission: {} bps", commission);
    println!("Slippage: {} bps\n", slippage);

    let engine = BacktestEngine::new(data, capital, execution_model);

    println!("Running backtest...\n");
    let result = engine.run_buy_and_hold();

    println!("=== RESULTS ===");
    println!("Initial capital: ${:>12.2}", result.initial_capital);
    println!("Final equity:    ${:>12.2}", result.final_equity);
    println!("Total return:    {:>11.2}%", result.total_return * 100.0);
    println!("Sharpe ratio:    {:>12.2}", result.sharpe_ratio);
    println!("Max drawdown:    {:>11.2}%", result.max_drawdown * 100.0);
    println!("Total trades:    {:>12}", result.total_trades);

    let output_path = PathBuf::from("results/backtests/buy_and_hold.json");
    match result.save_to_file(&output_path) {
        Ok(_) => println!("\nSaved to: {}", output_path.display()),
        Err(e) => eprintln!("Failed to save: {}", e),
    }
}
