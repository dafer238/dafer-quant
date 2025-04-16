use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};
use utils::get_project_cwd;

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
            use chrono::Local;

            let __start = std::time::Instant::now();
            let __timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let __module_path = module_path!();
            let __fn_name = stringify!(#name);
            let __full_name = format!("{}::{}", __module_path, __fn_name);

            let __result = std::panic::catch_unwind(|| {
                (|| #block)()
            });

            let __duration = __start.elapsed();
            let __log_line = match &__result {
                Ok(_) => format!(
                    "[{}] SUCCESS [{}] took {:?}",
                    __timestamp, __full_name, __duration
                ),
                Err(_) => format!(
                    "[{}] ERROR   [{}] panicked after {:?}",
                    __timestamp, __full_name, __duration
                ),
            };

            // In the macro-expanded code (or inside the macro-generated code block)
            let mut log_path = get_project_cwd();
            log_path.push("logs");

            // Now push the file name
            log_path.push("perf.log");

            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path)
            {
                let _ = writeln!(file, "{}", __log_line);
            }

            println!("{}", __log_line);

            match __result {
                Ok(val) => val,
                Err(err) => std::panic::resume_unwind(err),
            }
        }
    };

    output.into()
}
