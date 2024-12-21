use chrono::NaiveDateTime;
use polars::prelude::{DataFrame};
  
#[derive(Debug)] 
struct Position {
    iv: f64,
    av: f64,
    y: f64,
}

#[derive(Debug)]
struct Asset {
    isin: String,
    short_name: String,
    name: String,
    desc: String,
    positions: Position, //Eventually vector of positions from different people
    history_data: DataFrame,
}

enum Operation<'a> {
    Buy {
        datetime: NaiveDateTime,
        isin: &'a str,
        shares: f64,
        price: f64,
    },
    Sell {
        datetime: NaiveDateTime,
        isin: &'a str,
        shares: f64,
        price: f64,
    },
    Transfer {
        datetime: NaiveDateTime,
        isin_from: &'a str,
        isin_to: &'a str,
        shares_from: f64,
        price_from: f64,
        shares_to: f64,
        price_to: f64,
    },
}

fn print_operations_header() {
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

fn print_operation(movement: Operation) {
    match movement {
        Operation::Buy {
            datetime,
            isin,
            shares,
            price,
        } => {
            println!(
                "{:<22} {:<10} {:<12} {:>12.4} {:>12.4}",
                datetime.to_string(),
                "BUY",
                isin,
                shares,
                price
            );
        }
        Operation::Sell {
            datetime,
            isin,
            shares,
            price,
        } => {
            println!(
                "{:<22} {:<10} {:<12} {:>12.4} {:>12.4}",
                datetime.to_string(),
                "SELL",
                isin,
                shares,
                price
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
                price_to
            );
        }
    }
}

fn main() {
    let test_buy = Operation::Buy {
        datetime: NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
            .expect("Error"),
        isin: "AAPL",
        shares: 2.23,
        price: 303.23467,
    };
    let test_sell = Operation::Sell {
        datetime: NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
            .expect("Error"),
        isin: "SP500",
        shares: 129.3,
        price: 10287.2764128,
    };
    let test_transfer = Operation::Transfer {
        datetime: NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
            .expect("Error"),
        isin_from: "IE7632X23",
        isin_to: "LU7826315",
        shares_from: 23.221,
        price_from: 105.214,
        shares_to: 0.12,
        price_to: 5207.3,
    };
    print_operations_header();
    print_operation(test_buy);
    print_operation(test_sell);
    print_operation(test_transfer);
}
