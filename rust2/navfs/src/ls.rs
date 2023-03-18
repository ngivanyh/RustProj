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

pub fn ls(mut path: Option<String>) {    
    if path == None {
        match env::var("PWD") {
            Ok(val) => path = Some(val.to_string()),
            Err(e) => println!("Couldn't get HOME: {}", e),
        };
    }

    let new_path = path.unwrap();

    let mut dirs: Vec<String> = Vec::new();

    for file in fs::read_dir(&new_path).unwrap() {
        dirs.push(file.unwrap().path().display().to_string());
    }
    dirs.sort();
    
    for dir in dirs {
        let new_dir = dir.replace(&new_path, "");
        filetype(&dir, &new_dir);
    }

}