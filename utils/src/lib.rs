use std::env;
use std::path::PathBuf;

pub fn get_cwd() -> PathBuf {
    env::current_dir().expect("Failed to get current working directory")
}
