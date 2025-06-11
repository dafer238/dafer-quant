// ./algjsl/src/main.rs

// External imports

// Import local crates

// Import from within the crate

use algjsl::app_config::AppConfig;
use perf_macro::performance_log;

#[tokio::main]
#[performance_log]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = AppConfig::load()?;
    println!("Configuration loaded successfully. Config: {:.?}", config);

    let file_path = "./data/input/movements_dafer.csv";
    let _lf = algjsl::logic::data_reader::read_movements_csv(&file_path).unwrap();
    // println!("{:?}", lf.collect());

    Ok(())
}
