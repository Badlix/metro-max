use std::env;

pub fn version() -> usize {
    let test = 0;
    println!("Version {}", env!("CARGO_PKG_VERSION"));
    1
}