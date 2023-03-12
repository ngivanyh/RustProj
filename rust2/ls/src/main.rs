use std::{env, fs};
use text_colorizer::Colorize;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.len() != 1 {
        eprintln!("{}", "Error colleting args".red().bold());
        std::process::exit(1);
    }

    let mut dirs: Vec<String> = Vec::new();

    for file in fs::read_dir(&args[0]).unwrap() {
        dirs.push(file.unwrap().path().display().to_string());
    }
    dirs.sort();

    for dir in dirs {
        let new_dir = dir.replace(&args[0], "");
        if &new_dir[0..2] == "/." {
            println!("{}", new_dir.bold());
        } else {
            println!("{}", new_dir.cyan().bold());
        }
    }

}
