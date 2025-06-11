// ./algjsl/src/logic/data_reader.rs

use perf_macro::performance_log;
use polars::prelude::*;

#[performance_log]
pub fn read_movements_csv(file_path: &str) -> Result<LazyFrame, PolarsError> {
    println!("Reading movements from {}", file_path);
    let df = LazyCsvReader::new(file_path).finish()?;

    Ok(df)
}
