#![allow(dead_code)]

// Import local crates
use modules::owners;
use perf_macro::performance_log;
use utils::get_cwd;

// Import from within the crate
mod database;
mod modules;

// External imports
use crate::database::sqlite_db::Database;
use polars::prelude::*;

#[tokio::main]
#[performance_log]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let cwd = get_cwd();

    // Construct the file path relative to the CWD
    let file_path = cwd.join("data/input/hourly_data.csv");

    let shares_dani = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(file_path.into()))
        .expect("Failed to read CSV file")
        .finish()
        .expect("Failed to finish reading CSV");

    // Print the DataFrame
    // println!("{:?}", shares_dani);
    Ok(())
}
