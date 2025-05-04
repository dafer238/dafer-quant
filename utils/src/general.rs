// ./utils/src/general.rs

use std::env;
use std::fs;
use std::path::PathBuf;

pub fn get_cwd() -> PathBuf {
    env::current_dir().expect("Failed to get current working directory")
}

pub fn get_root_wd() -> PathBuf {
    let mut dir = env::current_dir().expect("Failed to get current directory");

    loop {
        let cargo_toml = dir.join("Cargo.toml");

        if cargo_toml.exists() {
            // Check if this Cargo.toml has a [workspace] section
            let content = fs::read_to_string(&cargo_toml).expect("Failed to read Cargo.toml");
            if content.contains("[workspace]") {
                return dir;
            }
        }

        // Stop if we reach filesystem root
        if !dir.pop() {
            panic!("Workspace root not found (no Cargo.toml with [workspace])");
        }
    }
}
