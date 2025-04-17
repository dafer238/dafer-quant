#![allow(unused)]
use perf_macro::performance_log;
use utils::get_cwd;

mod logic;
mod modules;

use modules::assets::{Owner, Position};
use polars::prelude::*;

#[performance_log]
fn main() {
    println!("");
    let mut dani = Owner::new(1, "Dani".to_string());
    let kepa = Owner::new(2, "Kepa".to_string());
    let gorka = Owner::new(3, "Gorka".to_string());

    println!("{:?}", dani);
    println!("{:?}", kepa);
    println!("{:?}", gorka);

    dani.add_asset(1);
    dani.add_assets(vec![3, 5]);

    println!("{:?}", dani);

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
    //println!("{:?}", shares_dani);

    let positions_dani = Position::new("AAPL".to_string(), 1, shares_dani);

    println!("{:?}", positions_dani)
}
