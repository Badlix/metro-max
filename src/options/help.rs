use colored::Colorize;

// TODO : unused for now, to reimplement if clap can't display custom colorful output

#[allow(dead_code)]
pub fn help() -> usize {
    println!();
    println!("{}", "A rust CLI for music !".blue());
    println!();
    println!("{}", "Options : ".green());
    println!("\t-h, --help : this message, use metromax [option] -h, to have more detail explanation on a certain option");
    println!("\t-v, --version : display current version");
    println!();
    println!("{}", "Commands : ".green());
    println!("\tlaunch [bpm] : launch a metronom with a given bpm value");
    println!();
    1
}