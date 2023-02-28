use std::{env, fs};
use text_colorizer::Colorize;

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.len() != 1 {
        eprintln!("{}", "Error: Expected 1, got more than one command line arguments".red().bold());
        std::process::exit(1)
    }
    
    let file_data = fs::metadata(&args[0])?;

    println!("File size {}", file_data.len());
   
    let contents = fs::read_to_string(&args[0]);

    
    Ok(())
}
