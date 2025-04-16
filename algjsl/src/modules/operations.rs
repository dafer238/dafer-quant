use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Enum to denote what kind of operation is being made.
pub enum Operation {
    Buy {
        datetime: NaiveDateTime,
        isin: String,
        shares: f64,
        price: f64,
        transaction_comission: f64,
    },
    Sell {
        datetime: NaiveDateTime,
        isin: String,
        shares: f64,
        price: f64,
        transaction_comission: f64,
    },
    Transfer {
        datetime: NaiveDateTime,
        isin_from: String,
        isin_to: String,
        shares_from: f64,
        price_from: f64,
        shares_to: f64,
        price_to: f64,
        transaction_comission: f64,
    },
}

impl Operation {
    pub fn buy(
        datetime: NaiveDateTime,
        isin: String,
        shares: f64,
        price: f64,
        transaction_comission: Option<f64>,
    ) -> Self {
        Operation::Buy {
            datetime,
            isin,
            shares,
            price,
            transaction_comission: transaction_comission.unwrap_or(0.0),
        }
    }
}

pub fn print_operations_header() {
    println!(
        "\n{:<22} {:<10} {:<12} {:>12} {:>12}\n{}",
        "DATE",
        "OP",
        "ISIN",
        "SHARES",
        "PRICE",
        "=".repeat(72)
    );
}

pub fn print_operation(movement: Operation) {
    match movement {
        Operation::Buy {
            datetime,
            isin,
            shares,
            price,
            transaction_comission,
        } => {
            println!(
                "{:<22} {:<10} {:<12} {:>12.4} {:>12.4}",
                datetime.to_string(),
                "BUY",
                isin,
                shares,
                price,
                // transaction_comission,
            );
        }
        Operation::Sell {
            datetime,
            isin,
            shares,
            price,
            transaction_comission,
        } => {
            println!(
                "{:<22} {:<10} {:<12} {:>12.4} {:>12.4}",
                datetime.to_string(),
                "SELL",
                isin,
                shares,
                price,
                // transaction_comission,
            );
        }
        Operation::Transfer {
            datetime,
            isin_from,
            isin_to,
            shares_from,
            price_from,
            shares_to,
            price_to,
            transaction_comission,
        } => {
            println!(
                "{:<22} {:<10} {:<12} {:>12.4} {:>12.4}\n{:<22} {:<10} {:<12} {:>12.4} {:>12.4}",
                datetime.to_string(),
                "SEND",
                isin_from,
                shares_from,
                price_from,
                datetime.to_string(),
                "RECEIVE",
                isin_to,
                shares_to,
                price_to,
                // transaction_comission,
            );
        }
    }
}
