use polars::prelude::*;
use utils::plotting::hist_plot::plot_candlestick;

#[test]
fn test_plot_candlestick_runs() {
    // Get the root working directory
    let root_wd = utils::general::get_root_wd();

    // Build a minimal valid DataFrame for candlestick plotting
    let file_path = root_wd.join("data/input/histdata_sp500.csv");

    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(file_path.into()))
        .expect("Failed to read SP500 CSV")
        .finish()
        .expect("Failed to finish reading SP500 CSV");

    // Optionally redirect plots to a temp directory if the function saves files
    let result = plot_candlestick(&df, "Test plot");

    assert!(result.is_ok(), "Plotting candlestick chart should not fail");
}
