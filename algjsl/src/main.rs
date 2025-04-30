#![allow(dead_code)]

// External imports
use polars::prelude::*;

// Import local crates
use crate::database::sqlite_db::Database;
use crate::modules::owners;
// Import from within the crate
mod app_config;
mod database;
mod modules;

use app_config::AppConfig;

use perf_macro::performance_log;
use utils::plotting::hist_plot::{
    plot_candlestick,
    // plot_line,
    // plot_scatter
};

#[tokio::main]
#[performance_log]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = AppConfig::load()?;
    println!("Configuration loaded successfully. Config: {:.?}", config);

    // Load .env vars
    dotenvy::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    // Initialize the database
    let db = Database::new(&db_url).await?;
    db.init_db().await?;

    let dani = modules::owners::Owner::new(
        "dafer".to_string(),
        "dani@gmail.com".to_string(),
        "123abc".to_string(),
        Some(modules::owners::UserGroup::Trader),
        None,
    );

    // unwrap allows you to process its output, ok just works when no Err is returned.
    owners::Owner::create_owner(&db, &dani).await.ok();

    let owners = owners::Owner::get_all_owners(&db);

    println!("Database owners:");
    println!("{:?}", owners.await.unwrap());

    // Get the current working directory (CWD) where the program is run from
    let cwd = utils::general::get_cwd();

    // Construct the file path relative to the CWD
    let movements_file_path = cwd.join("data/input/movements_dafer_UTF8.csv");

    let movements_dani = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(movements_file_path.into()))
        .expect("Failed to read CSV file")
        .finish()
        .expect("Failed to finish reading CSV");

    println!("{:?}", movements_dani);

    let hist_file_path = cwd.join("data/input/histdata_sp500.csv");
    let df_sp500 = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(hist_file_path.into()))
        .expect("Failed to read CSV file")
        .finish()
        .expect("Failed to finish reading CSV");

    // Plot a dataframe with proper candlestick information
    // let _ = plot_candlestick(&df_sp500, "Test plot");

    Ok(())
}
