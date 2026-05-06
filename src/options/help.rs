use colored::Colorize;

pub fn help() -> usize {
    println!();
    println!("{}", "A rust CLI for music !".blue());
    println!();
    println!("{}", "Options : ".green());
    println!("\t-h : this message, use metromax [command] -h, to have more detail explanation on a certain command");
    println!();
    1
}