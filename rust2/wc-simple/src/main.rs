use std::{env, fs};
use text_colorizer::Colorize;

fn main() { 
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.len() != 1 {
        eprintln!("{}", "Error: Expected 1, got more than one command line arguments".red().bold());
        std::process::exit(1);
    }
    
    let contents = fs::read_to_string(&args[0]).expect("Failed to read contents");

    let mut words: Vec<&str> = contents.split_whitespace().collect();
    

