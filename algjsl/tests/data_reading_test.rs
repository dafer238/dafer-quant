// ./algjsl/tests/data_reading_test.rs

use polars::prelude::*;

#[test]
fn test_csv_movements_loading() {
    // Get the root working directory
    let root_wd = utils::general::get_root_wd();

    // Build path for first CSV
    let movements_file_path = root_wd.join("data/input/movements_dafer_UTF8.csv");

    let movements_dani = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(movements_file_path.into()))
        .expect("Failed to read movements CSV")
        .finish()
        .expect("Failed to finish reading movements CSV");

    assert!(
        movements_dani.height() > 0,
        "DataFrame 'movements_dani' is empty"
    );
}
#[test]
fn test_csv_hist_loading() {
    // Get the root working directory
    let root_wd = utils::general::get_root_wd();
    // Build path for second CSV
    let hist_file_path = root_wd.join("data/input/histdata_sp500.csv");

    let df_sp500 = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(hist_file_path.into()))
        .expect("Failed to read SP500 CSV")
        .finish()
        .expect("Failed to finish reading SP500 CSV");

    assert!(df_sp500.height() > 0, "DataFrame 'df_sp500' is empty");
}
