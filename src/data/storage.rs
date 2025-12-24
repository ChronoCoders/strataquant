use crate::data::types::OHLCV;
use anyhow::{Context, Result};
use polars::prelude::*;
use std::path::Path;

pub fn save_to_parquet(data: &[OHLCV], path: &Path) -> Result<()> {
    let timestamps: Vec<i64> = data.iter().map(|d| d.timestamp).collect();
    let opens: Vec<f64> = data.iter().map(|d| d.open).collect();
    let highs: Vec<f64> = data.iter().map(|d| d.high).collect();
    let lows: Vec<f64> = data.iter().map(|d| d.low).collect();
    let closes: Vec<f64> = data.iter().map(|d| d.close).collect();
    let volumes: Vec<f64> = data.iter().map(|d| d.volume).collect();

    let df = DataFrame::new(vec![
        Column::Series(Series::new("timestamp".into(), timestamps)),
        Column::Series(Series::new("open".into(), opens)),
        Column::Series(Series::new("high".into(), highs)),
        Column::Series(Series::new("low".into(), lows)),
        Column::Series(Series::new("close".into(), closes)),
        Column::Series(Series::new("volume".into(), volumes)),
    ])
    .context("Failed to create DataFrame")?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = std::fs::File::create(path)?;
    ParquetWriter::new(&mut file).finish(&mut df.clone())?;

    Ok(())
}

pub fn load_from_parquet(path: &Path) -> Result<Vec<OHLCV>> {
    let file = std::fs::File::open(path)
        .context(format!("Failed to open file: {}", path.display()))?;

    let df = ParquetReader::new(file)
        .finish()
        .context("Failed to read Parquet file")?;

    let timestamps = df
        .column("timestamp")
        .context("Missing timestamp column")?
        .i64()
        .context("Invalid timestamp type")?;

    let opens = df
        .column("open")
        .context("Missing open column")?
        .f64()
        .context("Invalid open type")?;

    let highs = df
        .column("high")
        .context("Missing high column")?
        .f64()
        .context("Invalid high type")?;

    let lows = df
        .column("low")
        .context("Missing low column")?
        .f64()
        .context("Invalid low type")?;

    let closes = df
        .column("close")
        .context("Missing close column")?
        .f64()
        .context("Invalid close type")?;

    let volumes = df
        .column("volume")
        .context("Missing volume column")?
        .f64()
        .context("Invalid volume type")?;

    let mut data = Vec::new();
    for i in 0..df.height() {
        data.push(OHLCV::new(
            timestamps.get(i).context("Missing timestamp")?,
            opens.get(i).context("Missing open")?,
            highs.get(i).context("Missing high")?,
            lows.get(i).context("Missing low")?,
            closes.get(i).context("Missing close")?,
            volumes.get(i).context("Missing volume")?,
        ));
    }

    Ok(data)
}
