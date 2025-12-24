use crate::data::types::OHLCV;
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct BinanceKline(
    i64,    // Open time
    String, // Open
    String, // High
    String, // Low
    String, // Close
    String, // Volume
    i64,    // Close time
    String, // Quote asset volume
    u64,    // Number of trades
    String, // Taker buy base asset volume
    String, // Taker buy quote asset volume
    String, // Ignore
);

pub struct BinanceDownloader {
    client: Client,
    symbol: String,
    interval: String,
}

impl BinanceDownloader {
    pub fn new(symbol: &str, interval: &str) -> Self {
        Self {
            client: Client::new(),
            symbol: symbol.to_string(),
            interval: interval.to_string(),
        }
    }

    pub fn fetch_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<OHLCV>> {
        let mut all_data = Vec::new();
        let mut current = start;

        let chunk_size = match self.interval.as_str() {
            "1d" => Duration::days(1000),
            "1h" => Duration::hours(1000),
            "5m" => Duration::minutes(5000),
            "1m" => Duration::minutes(1000),
            _ => Duration::days(1000),
        };

        println!("Fetching data in chunks...");

        while current < end {
            let chunk_end = (current + chunk_size).min(end);

            let url = format!(
                "https://api.binance.us/api/v3/klines?symbol={}&interval={}&startTime={}&endTime={}&limit=1000",
                self.symbol,
                self.interval,
                current.timestamp_millis(),
                chunk_end.timestamp_millis()
            );

            let response: Vec<BinanceKline> = self
                .client
                .get(&url)
                .send()
                .context("Failed to fetch data from Binance")?
                .json()
                .context("Failed to parse response")?;

            if response.is_empty() {
                break;
            }

            print!(".");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();

            for kline in response {
                all_data.push(OHLCV::new(
                    kline.0,
                    kline.1.parse().context("Failed to parse open price")?,
                    kline.2.parse().context("Failed to parse high price")?,
                    kline.3.parse().context("Failed to parse low price")?,
                    kline.4.parse().context("Failed to parse close price")?,
                    kline.5.parse().context("Failed to parse volume")?,
                ));
            }

            current = chunk_end;
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        println!();
        Ok(all_data)
    }
}
