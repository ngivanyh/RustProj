use std::{env, fs};
use text_colorizer::Colorize;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.len() != 1 {
        eprintln!("{}", "Error colleting args".red().bold());
        std::process::exit(1);
    }

    println!("path: {}", args[0]);

    let mut dirs: Vec<&str> = Vec::new();

    for file in fs::read_dir(&args[0]).unwrap() {
        dirs.push(file.unwrap().path().display().to_string());
        println!("{}", file.unwrap().path().display());
    }
}
