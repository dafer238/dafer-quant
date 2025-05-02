// External imports

// Import local crates

// Import from within the crate
pub mod app_config;
pub mod database;
pub mod errors;
pub mod modules;

use app_config::AppConfig;
use perf_macro::performance_log;

#[tokio::main]
#[performance_log]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = AppConfig::load()?;
    println!("Configuration loaded successfully. Config: {:.?}", config);

    Ok(())
}
