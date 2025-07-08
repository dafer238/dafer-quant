// ./jsl_core/src/main.rs

// External imports

// Import local crates

// Import from within the crate

use jsl_core::app_config::AppConfig;
use jsl_perf::jsl_performance_log;

#[tokio::main]
#[jsl_performance_log(mode="print")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let _config = AppConfig::load()?;
    // println!("Configuration loaded successfully. Config: {:.?}", config);

    // println!("Reading movements from csv");
    let file_path = "./data/input/movements_dafer.csv";

    // println!("Reading movements from {}", file_path);
    let _lf = jsl_core::logic::data_reader::read_movements_csv(&file_path).unwrap();
    // println!("{:?}", _lf.collect());

    Ok(())
}
