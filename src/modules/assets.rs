#[allow(unused)]
use polars::prelude::DataFrame;

#[derive(Debug, Clone)]
/// Struct to represent the owner of an asset (e.g., person holding the stock).
pub struct Owner {
    name: String,
    id: u32, // Unique identifier for the owner
}

impl Owner {
    pub fn new(name: String, id: u32) -> Self {
        Self { name, id }
    }
}

#[derive(Debug, Clone)]
/// Struct to define a Position in an Asset, owned by an Owner
pub struct Position {
    isin: String,
    owner: Owner,
    shares: f64, // To be a series of Datetime \ Shares bought \ Shares acc.
    actual_price: f64,
    invested_value: f64,
    actual_value: f64,
    yld: f64,
}

impl Position {
    pub fn new(isin: String, owner: Owner, shares: f64, actual_price: f64) -> Self {
        let invested_value: f64 = 1.0; // TODO: Matrix multiplication of shares times buy price
        let actual_value: f64 = shares * actual_price;
        let yld: f64 = (actual_value - invested_value) / invested_value;

        Self {
            isin,
            owner,
            shares,
            actual_price,
            invested_value,
            actual_value,
            yld,
        }
    }
}

#[derive(Debug, Clone)]
/// Struct to define an Asset univocally, keeping its information as well.
pub struct Asset {
    isin: String,
    short_name: String,
    name: String,
    desc: String,
    positions: Vec<Position>,
    history_data: DataFrame,
}
