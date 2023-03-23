use std::{env, fs};
use std::path::PathBuf;

pub fn ls(mut path: Option<String>) -> Vec<String> {    
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
    
    dirs
}
