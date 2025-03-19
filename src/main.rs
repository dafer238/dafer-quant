#![allow(unused)]

mod logic;
mod modules;

use modules::assets::{Owner, Position};
use polars_core::prelude::*;
use polars_io::prelude::*;
use std::{fs::File, ptr::read};

fn main() {
    println!("");
    let mut dani = Owner::new(1, "Dani".to_string());
    let mut kepa = Owner::new(2, "Kepa".to_string());
    let mut gorka = Owner::new(3, "Gorka".to_string());

    println!("{:?}", dani);
    println!("{:?}", kepa);
    println!("{:?}", gorka);

    dani.add_asset("AAPL".to_string());
    dani.add_assets(vec!["LMT".to_string(), "MSFT".to_string()]);

    println!("{:?}", dani);

    // Path to the CSV file
    let file_path = "hourly_data.csv";

    let shares_dani = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(file_path.into()))
        .expect("Failed to read CSV file")
        .finish()
        .expect("Failed to finish reading CSV");

    // Print the DataFrame
    //println!("{:?}", shares_dani);

    let mut positions_dani = Position::new("AAPL".to_string(), dani, shares_dani);

    println!("{:?}", positions_dani)
}
