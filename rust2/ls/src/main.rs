use std::{env, fs};
use std::path::PathBuf;
use text_colorizer::Colorize;

fn filetype(dir: &str, print_dir: &str) {
    let path = PathBuf::from(dir);
    if  path.is_dir() {
        println!("{}", print_dir.blue().bold());
    } else {
        println!("{}", print_dir.bold());
    }
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    
    if args.len() != 1 {
        match env::var("PWD") {
            Ok(val) => args.push(val),
            Err(e) => println!("Couldn't get HOME: {}", e),
        };
    }

    let mut dirs: Vec<String> = Vec::new();

    for file in fs::read_dir(&args[0]).unwrap() {
        dirs.push(file.unwrap().path().display().to_string());
    }
    dirs.sort();
    
    for dir in dirs {
        let new_dir = dir.replace(&args[0], "");
        filetype(&dir, &new_dir);
    }

}
