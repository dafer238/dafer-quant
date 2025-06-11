// ./perf_macro/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn performance_log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let name = &sig.ident;
    let output = quote! {
        #vis #sig {
            use std::fs::OpenOptions;
            use std::io::Write;
            use chrono::Utc;
            use std::panic;
            let __start = std::time::Instant::now();
            let __timestamp = Utc::now().to_rfc3339();
            let __module_path = module_path!();
            let __fn_name = stringify!(#name);
            let __full_name = format!("{}::{}", __module_path, __fn_name);
            let mut log_path = ::utils::general::get_root_wd();
            log_path.push("logs");
            log_path.push("perf.log");
            let __result = panic::catch_unwind(|| (|| #block)());
            let __duration = __start.elapsed();

            // Format duration with fixed width
            let __duration_str = {
                let secs = __duration.as_secs();
                let nanos = __duration.subsec_nanos();

                if secs > 0 {
                    // If more than a second, format as X.XXXs
                    let decimal = nanos / 1_000_000; // First 3 digits of nanos (milliseconds)
                    format!("{}.{:03}s", secs, decimal)
                } else {
                    let micros = nanos / 1_000;
                    if micros >= 1_000 {
                        // If more than a millisecond but less than a second, format as XXX.XXXms
                        let ms = micros / 1_000;
                        let ms_decimal = micros % 1_000;

                        // Make sure it's always formatted as 999.999ms pattern (fixed width)
                        format!("{:3}.{:03}ms", ms, ms_decimal)
                    } else {
                        // If less than a millisecond, format as XXX.XXXμs
                        let us = micros;
                        let us_decimal = nanos % 1_000;

                        // Make sure it's always formatted as 999.999μs pattern (fixed width)
                        format!("{:3}.{:03}μs", us, us_decimal)
                    }
                }
            };

            // Prepare plain status and colored status strings
            let __status_plain = match &__result {
                Ok(Ok(_)) => "SUCCESS",
                Ok(Err(_)) => "ERROR  ",
                Err(_) => "PANIC  ",
            };
            let __status_colored = match &__result {
                Ok(Ok(_)) => "\x1b[1m\x1b[32mSUCCESS\x1b[0m", // bold green
                Ok(Err(_)) => "\x1b[1m\x1b[33mERROR  \x1b[0m",   // bold yellow
                Err(_) => "\x1b[1m\x1b[31mPANIC  \x1b[0m",       // bold red
            };
            // Prepare the message parts
            let __log_message = match &__result {
                Ok(Ok(_)) => "ran successfully".to_string(),
                Ok(Err(err)) => format!("returned error: {:?}", err),
                Err(_) => "panicked.".to_string(),
            };
            // Build the plain log line (for file)
            let __log_line_plain = format!(
                "[{}] {} [{}] {} {}",
                __timestamp,
                __status_plain,
                __duration_str,
                __full_name,
                __log_message,
            );
            // Build the colored log line (for console)
            let __log_line_colored = format!(
                "[{}] {} [{}] {} {}",
                __timestamp,
                __status_colored,
                __duration_str,
                __full_name,
                __log_message,
            );
            // Write the plain log line to file (no ANSI codes)
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path)
            {
                let _ = writeln!(file, "{}", __log_line_plain);
            }
            // Print colored line to console
            println!("{}", __log_line_colored);
            // Return or resume panic as usual
            match __result {
                Ok(val) => val,
                Err(err) => panic::resume_unwind(err),
            }
        }
    };
    output.into()
}
