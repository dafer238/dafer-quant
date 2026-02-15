// ./pmm-core/src/logic/data_reader.rs

use pmm_perf::performance_log;
use polars::prelude::*;

#[performance_log(mode = "print")]
pub fn read_movements_csv(file_path: &str) -> Result<LazyFrame, PolarsError> {
    let df = LazyCsvReader::new(file_path.into()).finish()?;
    Ok(df)
}
