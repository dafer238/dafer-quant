// ./pmm-core/src/main.rs

// External imports

// Import local crates

// Import from within the crate

use pmm_core::app_config::AppConfig;
use pmm_perf::performance_log;

// #[tokio::main]
#[performance_log(mode = "both")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = AppConfig::load()?;
    println!("Configuration loaded successfully. Config: {:.?}", config);

    let file_path = "./data/input/movements.csv";
    println!("Reading movements from {}", file_path);
    let lf = pmm_core::logic::data_reader::read_movements_csv(file_path).unwrap();
    println!("{:?}", lf.collect());

    Ok(())
}
