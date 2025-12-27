use crate::backtest::BacktestResult;
use plotters::prelude::*;
use std::path::Path;

pub fn plot_equity_curve(
    result: &BacktestResult,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_path, (1200, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_equity = result
        .equity_curve
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let max_equity = result
        .equity_curve
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Equity Curve", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(80)
        .build_cartesian_2d(0..result.equity_curve.len(), min_equity..max_equity)?;

    chart
        .configure_mesh()
        .x_desc("Days")
        .y_desc("Equity ($)")
        .draw()?;

    chart.draw_series(LineSeries::new(
        result.equity_curve.iter().enumerate().map(|(i, &v)| (i, v)),
        &BLUE,
    ))?;

    root.present()?;
    Ok(())
}

pub fn plot_drawdown(
    result: &BacktestResult,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_path, (1200, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut peak = result.initial_capital;
    let mut drawdowns = Vec::new();

    for &equity in &result.equity_curve {
        if equity > peak {
            peak = equity;
        }
        let dd = ((equity - peak) / peak) * 100.0;
        drawdowns.push(dd);
    }

    let min_dd = drawdowns.iter().copied().fold(f64::INFINITY, f64::min);

    let mut chart = ChartBuilder::on(&root)
        .caption("Drawdown", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(80)
        .build_cartesian_2d(0..drawdowns.len(), min_dd..0.0)?;

    chart
        .configure_mesh()
        .x_desc("Days")
        .y_desc("Drawdown (%)")
        .draw()?;

    chart.draw_series(AreaSeries::new(
        drawdowns.iter().enumerate().map(|(i, &v)| (i, v)),
        0.0,
        RED.mix(0.3),
    ))?;

    root.present()?;
    Ok(())
}
