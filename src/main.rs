use colored::Colorize;
use std::env;
use clap::{Parser, Subcommand};
mod options;
mod commands;

#[derive(Parser)]
#[command(version, about = "A rust CLI for music !")]
struct SimpleCLI {
    #[command(subcommand)]
    command: CommandsEnum
}

#[derive(Subcommand)]
enum CommandsEnum {
    Start(commands::start::StartArgs)
}

#[derive(Parser)]
#[command(version, about = "A rust CLI for music !")]
struct SimpleCLI {}

fn main() {
    let cli = SimpleCLI::parse();

    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        println!("{}", "METROMAX".blue().bold());
        println!("\n{}", "See --help".blue().bold());
    }

    match &cli.command {
        CommandsEnum::Start(args) => {
            commands::start::start(args.bpm, args.beat_number, args.big_key_index.clone());
        }
    }
}