use std::env;

pub fn version() -> usize {
        println!("Version {}", env!("CARGO_PKG_VERSION"));
    1
}