use crate::data::types::OHLCV;
use polars::prelude::*;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Polars error: {0}")]
    PolarsError(#[from] PolarsError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn save_to_parquet(data: &[OHLCV], path: &Path) -> Result<(), StorageError> {
    let timestamps: Vec<i64> = data.iter().map(|d| d.timestamp).collect();
    let opens: Vec<f64> = data.iter().map(|d| d.open).collect();
    let highs: Vec<f64> = data.iter().map(|d| d.high).collect();
    let lows: Vec<f64> = data.iter().map(|d| d.low).collect();
    let closes: Vec<f64> = data.iter().map(|d| d.close).collect();
    let volumes: Vec<f64> = data.iter().map(|d| d.volume).collect();

    let df = DataFrame::new(vec![
        Series::new("timestamp", timestamps),
        Series::new("open", opens),
        Series::new("high", highs),
        Series::new("low", lows),
        Series::new("close", closes),
        Series::new("volume", volumes),
    ])?;

    let mut file = std::fs::File::create(path)?;
    ParquetWriter::new(&mut file).finish(&mut df.clone())?;

    Ok(())
}

pub fn load_from_parquet(path: &Path) -> Result<Vec<OHLCV>, StorageError> {
    let file = std::fs::File::open(path)?;
    let df = ParquetReader::new(file).finish()?;

    let timestamps = df
        .column("timestamp")?
        .i64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();
    let opens = df
        .column("open")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();
    let highs = df
        .column("high")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();
    let lows = df
        .column("low")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();
    let closes = df
        .column("close")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();
    let volumes = df
        .column("volume")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();

    let mut data = Vec::new();
    for i in 0..timestamps.len() {
        data.push(OHLCV {
            timestamp: timestamps[i],
            open: opens[i],
            high: highs[i],
            low: lows[i],
            close: closes[i],
            volume: volumes[i],
        });
    }

    Ok(data)
}
