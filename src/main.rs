use colored::Colorize;
use std::env;
mod options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len: usize = args.len();

    if args_len == 1 {
        println!("{}", "Metromax".blue().bold());
        println!("\n{}", "See --help".blue().bold());
        return;
    }
    
    let mut next_arg_index: usize = 1;
    while next_arg_index < args_len {
        let fn_return_val = parse(args.clone(), next_arg_index);
        if fn_return_val == 0 {
            break;
        }
        next_arg_index += fn_return_val;
    }
}

fn parse(args: Vec<String>, index: usize) -> usize {
    match args[index].as_str() {
        "-v" | "--version" => options::version::version(),
        "-h" | "--help" => options::help::help(),
        _ => 1
    }
}