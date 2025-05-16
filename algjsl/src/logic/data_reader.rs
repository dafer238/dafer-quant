// ./algjsl/src/logic/data_reader.rs

use perf_macro::performance_log;
use polars::prelude::*;

#[performance_log]
pub fn read_movements_csv(file_path: &str) -> Result<DataFrame, PolarsError> {
    println!("Reading movements from {}", file_path);
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(file_path.into()))?
        .finish()?;

    Ok(df)
}
