/// A enum to classify operations.
enum Operation<'a> {
    Buy {
        isin: &'a str, // Reference with a lifetime 'a
        shares: f64,
        price: f64,
    },
    Sell {
        isin: &'a str, // Reference with a lifetime 'a
        shares: f64,
        price: f64,
    },
    Transfer {
        isin_from: &'a str, // Reference with a lifetime 'a
        isin_to: &'a str,   // Reference with a lifetime 'a
        shares_from: f64,
        price_from: f64,
        shares_to: f64,
        price_to: f64,
    },
}

fn print_operation(movement: Operation) {
    println!(
        "\n{:<10} {:<15} {:>8} {:>12}\n{:<45}",
        "OP", "ISIN", "SHARES", "PRICE", "================================================"
    );
    match movement {
        Operation::Buy {
            isin,
            shares,
            price,
        } => {
            println!("{:<10} {:<15} {:>8.4} {:>12.4}", "BUY", isin, shares, price);
        }
        Operation::Sell {
            isin,
            shares,
            price,
        } => {
            println!(
                "{:<10} {:<15} {:>8.4} {:>12.4}",
                "SELL", isin, shares, price
            );
        }
        Operation::Transfer {
            isin_from,
            isin_to,
            shares_from,
            price_from,
            shares_to,
            price_to,
        } => {
            println!(
                "{:<10} {:<15} {:>8.4} {:>12.4}\n{:<10} {:<15} {:>8.4} {:>12.4}",
                "SEND", isin_from, shares_from, price_from, "RECEIVE", isin_to, shares_to, price_to
            );
        }
    }
}

fn main() {
    let test_buy = Operation::Buy {
        isin: "AAPL",
        shares: 2.23,
        price: 303.23467,
    };
    let test_sell = Operation::Sell {
        isin: "SP500",
        shares: 129.3,
        price: 10287.2764128,
    };
    let test_transfer = Operation::Transfer {
        isin_from: "IE7632X23",
        isin_to: "LU7826315",
        shares_from: 23.221,
        price_from: 105.214,
        shares_to: 0.12,
        price_to: 5207.3,
    };
    print_operation(test_buy);
    print_operation(test_sell);
    print_operation(test_transfer);
}
