use crate::data::types::OHLCV;
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::thread::sleep;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("API request failed: {0}")]
    ApiError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Failed to parse {0}: {1}")]
    ParseError(String, String),

    #[error("Invalid data: {0}")]
    ValidationError(String),
}

#[derive(Debug, Deserialize)]
struct BinanceKline {
    #[serde(rename = "0")]
    open_time: i64,
    #[serde(rename = "1")]
    open: String,
    #[serde(rename = "2")]
    high: String,
    #[serde(rename = "3")]
    low: String,
    #[serde(rename = "4")]
    close: String,
    #[serde(rename = "5")]
    volume: String,
}

pub struct BinanceDownloader {
    client: Client,
    symbol: String,
    interval: String,
    max_retries: u32,
    api_base: String,
}

impl BinanceDownloader {
    pub fn new(symbol: &str, interval: &str) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap(),
            symbol: symbol.to_string(),
            interval: interval.to_string(),
            max_retries: 3,
            api_base: "https://api.binance.us".to_string(),
        }
    }

    pub fn fetch_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<OHLCV>, DataError> {
        let mut all_data = Vec::new();
        let mut current = start;

        let interval_ms = self.interval_to_milliseconds();
        let chunk_ms = interval_ms * 1000;

        let total_ms = (end.timestamp_millis() - start.timestamp_millis()) as u64;
        let total_chunks = (total_ms as f64 / chunk_ms as f64).ceil() as usize;

        println!(
            "Downloading {} chunks of up to 1000 candles each",
            total_chunks
        );

        let mut chunk_count = 0;

        while current < end {
            chunk_count += 1;

            let chunk_end_ms = current.timestamp_millis() + chunk_ms as i64;
            let chunk_end = DateTime::from_timestamp_millis(chunk_end_ms)
                .unwrap_or(end)
                .min(end);

            let klines = self.fetch_chunk_with_retry(current, chunk_end)?;

            if klines.is_empty() {
                println!("No more data available at {}", current);
                break;
            }

            for kline in klines {
                let ohlcv = OHLCV {
                    timestamp: kline.open_time,
                    open: kline
                        .open
                        .parse()
                        .map_err(|e| DataError::ParseError("open".into(), format!("{}", e)))?,
                    high: kline
                        .high
                        .parse()
                        .map_err(|e| DataError::ParseError("high".into(), format!("{}", e)))?,
                    low: kline
                        .low
                        .parse()
                        .map_err(|e| DataError::ParseError("low".into(), format!("{}", e)))?,
                    close: kline
                        .close
                        .parse()
                        .map_err(|e| DataError::ParseError("close".into(), format!("{}", e)))?,
                    volume: kline
                        .volume
                        .parse()
                        .map_err(|e| DataError::ParseError("volume".into(), format!("{}", e)))?,
                };

                if !ohlcv.is_valid() {
                    return Err(DataError::ValidationError(format!(
                        "Invalid OHLCV at timestamp {}",
                        ohlcv.timestamp
                    )));
                }

                all_data.push(ohlcv);
            }

            current = chunk_end;

            if chunk_count % 10 == 0 {
                println!(
                    "Progress: {}/{} chunks ({:.1}%)",
                    chunk_count,
                    total_chunks,
                    (chunk_count as f64 / total_chunks as f64) * 100.0
                );
            }

            sleep(Duration::from_millis(100));
        }

        println!("Downloaded {} total candles", all_data.len());
        Ok(all_data)
    }

    fn fetch_chunk_with_retry(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<BinanceKline>, DataError> {
        let mut attempts = 0;

        loop {
            attempts += 1;

            let url = format!(
                "{}/api/v3/klines?symbol={}&interval={}&startTime={}&endTime={}&limit=1000",
                self.api_base,
                self.symbol,
                self.interval,
                start.timestamp_millis(),
                end.timestamp_millis()
            );

            match self.client.get(&url).send() {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<Vec<Vec<serde_json::Value>>>() {
                            Ok(raw_klines) => {
                                let mut klines = Vec::new();
                                for raw in raw_klines {
                                    if raw.len() >= 6 {
                                        let kline = BinanceKline {
                                            open_time: raw[0].as_i64().unwrap_or(0),
                                            open: raw[1].as_str().unwrap_or("0").to_string(),
                                            high: raw[2].as_str().unwrap_or("0").to_string(),
                                            low: raw[3].as_str().unwrap_or("0").to_string(),
                                            close: raw[4].as_str().unwrap_or("0").to_string(),
                                            volume: raw[5].as_str().unwrap_or("0").to_string(),
                                        };
                                        klines.push(kline);
                                    }
                                }
                                return Ok(klines);
                            }
                            Err(e) => {
                                if attempts >= self.max_retries {
                                    return Err(DataError::ApiError(format!(
                                        "JSON parse failed after {} attempts: {}",
                                        attempts, e
                                    )));
                                }
                                println!("JSON parse error, retrying... (attempt {})", attempts);
                                sleep(Duration::from_secs(1));
                            }
                        }
                    } else {
                        let status = response.status();
                        let body = response.text().unwrap_or_default();

                        if attempts >= self.max_retries {
                            return Err(DataError::ApiError(format!(
                                "HTTP {} after {} attempts: {}",
                                status, attempts, body
                            )));
                        }

                        println!("HTTP error {}, retrying... (attempt {})", status, attempts);
                        sleep(Duration::from_secs(2));
                    }
                }
                Err(e) => {
                    if attempts >= self.max_retries {
                        return Err(DataError::NetworkError(format!(
                            "Request failed after {} attempts: {}",
                            attempts, e
                        )));
                    }
                    println!("Network error, retrying... (attempt {})", attempts);
                    sleep(Duration::from_secs(2));
                }
            }
        }
    }

    fn interval_to_milliseconds(&self) -> u64 {
        match self.interval.as_str() {
            "1m" => 60_000,
            "5m" => 300_000,
            "15m" => 900_000,
            "1h" => 3_600_000,
            "4h" => 14_400_000,
            "1d" => 86_400_000,
            _ => panic!("Unsupported interval: {}", self.interval),
        }
    }
}
