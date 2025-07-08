// ./jsl_perf/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parser, parse_macro_input, punctuated::Punctuated, token::Comma, Expr, ExprLit, ItemFn,
    Lit, Meta,
};

#[proc_macro_attribute]
pub fn jsl_performance_log(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let args = Punctuated::<Meta, Comma>::parse_terminated
        .parse(attr)
        .unwrap_or_default();

    let mut log_to_console = true;
    let mut log_to_file = true;

    for meta in args {
        if let Meta::NameValue(nv) = meta {
            if nv.path.is_ident("mode") {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) = nv.value
                {
                    match lit_str.value().as_str() {
                        "none" => {
                            log_to_console = false;
                            log_to_file = false;
                        }
                        "file" => {
                            log_to_console = false;
                            log_to_file = true;
                        }
                        "print" => {
                            log_to_console = true;
                            log_to_file = false;
                        }
                        "both" => {
                            log_to_console = true;
                            log_to_file = true;
                        }
                        other => {
                            return syn::Error::new_spanned(
                                lit_str,
                                format!(
                                    "Invalid mode '{}'. Valid options: 'none', 'file', 'print', 'both'.",
                                    other
                                ),
                            )
                            .to_compile_error()
                            .into();
                        }
                    }
                }
            }
        }
    }

    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;
    let name = &sig.ident;

    let output = quote! {
        #vis #sig {
            use std::fs::OpenOptions;
            use std::io::Write;
            use chrono::Utc;
            use std::panic;

            let __log_to_console: bool = #log_to_console;
            let __log_to_file: bool = #log_to_file;

            let __start = std::time::Instant::now();
            let __timestamp = {
                let now = Utc::now();
                format!(
                    "{}.{:06}{}",
                    now.format("%Y-%m-%dT%H:%M:%S"),
                    now.timestamp_subsec_micros(),
                    "+00:00"
                )
            };
            let __module_path = module_path!();
            let __fn_name = stringify!(#name);
            let __full_name = format!("{}::{}", __module_path, __fn_name);

            let mut log_path = ::jsl_utils::general::get_root_wd();
            log_path.push("logs");
            log_path.push("perf.log");
            if let Some(logs_dir) = log_path.parent() {
                let _ = std::fs::create_dir_all(logs_dir);
            }

            let __result = panic::catch_unwind(|| (|| #block)());
            let __duration = __start.elapsed();

            let __duration_str = {
                let secs = __duration.as_secs();
                let nanos = __duration.subsec_nanos();

                if secs > 0 {
                    let decimal = nanos / 1_000_000;
                    format!("{}.{:03}s", secs, decimal)
                } else {
                    let micros = nanos / 1_000;
                    if micros >= 1_000 {
                        let ms = micros / 1_000;
                        let ms_decimal = micros % 1_000;
                        format!("{:3}.{:03}ms", ms, ms_decimal)
                    } else {
                        let us = micros;
                        let us_decimal = nanos % 1_000;
                        format!("{:3}.{:03}μs", us, us_decimal)
                    }
                }
            };

            let __status_plain = match &__result {
                Ok(Ok(_)) => "SUCCESS",
                Ok(Err(_)) => "ERROR  ",
                Err(_) => "PANIC  ",
            };

            let __status_colored = match &__result {
                Ok(Ok(_)) => "\x1b[1m\x1b[32mSUCCESS\x1b[0m",
                Ok(Err(_)) => "\x1b[1m\x1b[33mERROR  \x1b[0m",
                Err(_) => "\x1b[1m\x1b[31mPANIC  \x1b[0m",
            };

            let __log_message = match &__result {
                Ok(Ok(_)) => "ran successfully".to_string(),
                Ok(Err(err)) => format!("returned error: {:?}", err),
                Err(_) => "panicked.".to_string(),
            };

            let __log_line_plain = format!(
                "[{}] {} [{}] {} {}",
                __timestamp,
                __status_plain,
                __duration_str,
                __full_name,
                __log_message,
            );

            let __log_line_colored = format!(
                "[{}] {} [{}] {} {}",
                __timestamp,
                __status_colored,
                __duration_str,
                __full_name,
                __log_message,
            );

            if __log_to_file {
                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(log_path)
                {
                    let _ = writeln!(file, "{}", __log_line_plain);
                }
            }

            if __log_to_console {
                println!("{}", __log_line_colored);
            }

            match __result {
                Ok(val) => val,
                Err(err) => panic::resume_unwind(err),
            }
        }
    };

    output.into()
}
