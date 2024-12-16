use chrono::{NaiveDate, NaiveDateTime};

enum Operation<'a> {
    Buy {
        isin: &'a str,
        datetime: NaiveDateTime,
        shares: f64,
        price: f64,
    },
    Sell {
        isin: &'a str,
        datetime: NaiveDateTime,
        shares: f64,
        price: f64,
    },
    Transfer {
        isin_from: &'a str,
        isin_to: &'a str,
        datetime: NaiveDateTime,
        shares_from: f64,
        price_from: f64,
        shares_to: f64,
        price_to: f64,
    },
}

fn print_operations_header() {
    println!(
        "\n{:<10} {:<12} {:<20} {:>12} {:>12}\n{}",
        "OP",
        "ISIN",
        "DATE",
        "SHARES",
        "PRICE",
        "=".repeat(70)
    );
}

fn print_operation(movement: Operation) {
    match movement {
        Operation::Buy {
            isin,
            datetime,
            shares,
            price,
        } => {
            println!(
                "{:<10} {:<12} {:<20} {:>12.4} {:>12.4}",
                "BUY", isin, datetime, shares, price
            );
        }
        Operation::Sell {
            isin,
            datetime,
            shares,
            price,
        } => {
            println!(
                "{:<10} {:<12} {:<20} {:>12.4} {:>12.4}",
                "SELL", isin, datetime, shares, price
            );
        }
        Operation::Transfer {
            isin_from,
            isin_to,
            datetime,
            shares_from,
            price_from,
            shares_to,
            price_to,
        } => {
            println!(
                "{:<10} {:<12} {:<20} {:>12.4} {:>12.4}\n{:<10} {:<12} {:<20} {:>12.4} {:>12.4}",
                "SEND",
                isin_from,
                datetime,
                shares_from,
                price_from,
                "RECEIVE",
                isin_to,
                datetime,
                shares_to,
                price_to
            );
        }
    }
}

fn main() {
    let test_buy = Operation::Buy {
        isin: "AAPL",
        datetime: NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
            .expect("Error"),
        shares: 2.23,
        price: 303.23467,
    };
    let test_sell = Operation::Sell {
        isin: "SP500",
        datetime: NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
            .expect("Error"),
        shares: 129.3,
        price: 10287.2764128,
    };
    let test_transfer = Operation::Transfer {
        isin_from: "IE7632X23",
        isin_to: "LU7826315",
        datetime: NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
            .expect("Error"),
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
