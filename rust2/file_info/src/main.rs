use std::{env, fs};
use text_colorizer::Colorize;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("{}", "Error: You entered more than one or no cmd arguments".red().bold());
        std::process::exit(1);
    }
    
    println!("You searched for file {:?}", args);

    let metadata = fs::metadata(&args[0])?;
    

    println!("File size: {} bytes", metadata.len());
    println!("Permissions: {:?}", metadata.permissions());
    println!("Time created: {:?}", metadata.accessed());
    println!("Contents: {:?}", fs::read_to_string(&args[0]));

    Ok(())
}
