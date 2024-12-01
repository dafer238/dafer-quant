use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use polars::prelude::*;
use reqwest;
use serde_json::Value;

/// Configuration for fetching financial data
struct StockDataConfig {
    ticker: String,
    days_back: u32,
}

/// Fetch historical stock data using an alternative method
async fn fetch_stock_data(config: &StockDataConfig) -> Result<DataFrame> {
    // Construct URL for Yahoo Finance historical data
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{ticker}?range={days_back}d&interval=1d",
        ticker = config.ticker,
        days_back = config.days_back
    );

    // Send HTTP request
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .context("Failed to send request to Yahoo Finance")?;

    // Parse JSON response
    let json: Value = response
        .json()
        .await
        .context("Failed to parse JSON response")?;

    // Extract data from JSON
    let chart = json["chart"]["result"][0].clone();

    // Extract timestamps and quote data
    let timestamps: Vec<i64> = chart["timestamp"]
        .as_array()
        .context("No timestamp data found")?
        .iter()
        .filter_map(|t| t.as_i64())
        .collect();

    // Convert timestamps (u64) to NaiveDateTime
    let datetime_values: Vec<NaiveDateTime> = timestamps
        .iter()
        .map(|&timestamp| {
            DateTime::<Utc>::from_timestamp(timestamp as i64, 0)
                .expect("Error found")
                .naive_utc()
        })
        .collect();
    let quotes = chart["indicators"]["quote"][0].clone();

    // Extract quote data, handling potential missing values
    let opens: Vec<f64> = quotes["open"]
        .as_array()
        .context("No open price data found")?
        .iter()
        .filter_map(|v| v.as_f64())
        .collect();

    let highs: Vec<f64> = quotes["high"]
        .as_array()
        .context("No high price data found")?
        .iter()
        .filter_map(|v| v.as_f64())
        .collect();

    let lows: Vec<f64> = quotes["low"]
        .as_array()
        .context("No low price data found")?
        .iter()
        .filter_map(|v| v.as_f64())
        .collect();

    let closes: Vec<f64> = quotes["close"]
        .as_array()
        .context("No close price data found")?
        .iter()
        .filter_map(|v| v.as_f64())
        .collect();

    let volumes: Vec<u64> = quotes["volume"]
        .as_array()
        .context("No volume data found")?
        .iter()
        .filter_map(|v| v.as_u64())
        .collect();

    // Ensure all vectors have the same length
    let min_length = timestamps.len().min(
        opens.len().min(
            highs
                .len()
                .min(lows.len().min(closes.len().min(volumes.len()))),
        ),
    );

    // Create Polars DataFrame
    let df = DataFrame::new(vec![
        Series::new("timestamp", &timestamps[..min_length]),
        Series::new("datetime", &datetime_values[..min_length]),
        Series::new("open", &opens[..min_length]),
        Series::new("high", &highs[..min_length]),
        Series::new("low", &lows[..min_length]),
        Series::new("close", &closes[..min_length]),
        Series::new("volume", &volumes[..min_length]),
    ])?;

    Ok(df)
}

/// Main function to demonstrate usage
#[tokio::main]
async fn main() -> Result<()> {
    // Configure stock data fetch with start and stop dates
    let config = StockDataConfig {
        ticker: String::from("^GSPC"), // Apple stock
        days_back: 365,                // Fetch data for past year
    };

    // Fetch stock data
    let df = fetch_stock_data(&config).await?;

    // Display DataFrame information
    println!("Stock Data for {}", config.ticker);

    // Use polars' display method
    println!("{}", df);

    Ok(())
}
