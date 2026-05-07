use colored::Colorize;
use std::env;
use clap::Parser;
mod options;
mod commands;

#[derive(Parser)]
#[command(version, about = "A rust CLI for music !")]
struct SimpleCLI {}

fn main() {
    SimpleCLI::parse();

    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        println!("{}", "METROMAX".blue().bold());
        println!("\n{}", "See --help".blue().bold());
        return;
    }
}