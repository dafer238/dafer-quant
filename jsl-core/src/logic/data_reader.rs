// ./jsl_core/src/logic/data_reader.rs

use jsl_perf::jsl_performance_log;
use polars::prelude::*;

#[jsl_performance_log(mode = "print")]
pub fn read_movements_csv(file_path: &str) -> Result<LazyFrame, PolarsError> {
    let df = LazyCsvReader::new(file_path).finish()?;
    Ok(df)
}
