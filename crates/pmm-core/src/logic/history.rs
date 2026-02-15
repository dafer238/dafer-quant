// ./pmm-core/src/logic/history.rs

fn _compute_position_history() {}

// // Show the first few rows
// println!("First few rows:\n{}", df.head(Some(5)));
//
// // Filter rows where ACTION == "BUY"
// let buy_df = df
//     .clone()
//     .lazy()
//     .filter(col("ACTION").eq(lit("BUY")))
//     .collect()?;
// println!("Filtered DataFrame (ACTION = BUY):\n{}", buy_df);
//
// // Add new column TOTAL_COST = SHARES * VALUE
// let df_with_total = df
//     .clone()
//     .lazy()
//     .with_column((col("SHARES") * col("VALUE")).alias("TOTAL_COST"))
//     .collect()?;
//
// println!(
//     "DataFrame with TOTAL_COST column:\n{}",
//     df_with_total.head(Some(5))
// );
