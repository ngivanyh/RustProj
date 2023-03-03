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

    words.sort();

    let mut words_looped = 0;
    let mut wc = 0;

    while words_looped != words.len() - 1 {
        wc = 0;
        let mut n_word = words_looped + 1;
        if words[words_looped] == words[n_word] && n_word != words.len() - 1 && words_looped != words.len() - 1 {
            wc += 2;
            words_looped += 1;
            n_word += 1;
            while n_word != words.len() - 1 && words_looped != words.len() - 1 && words[n_word] == words[words_looped] {
                wc += 1;
                words_looped += 1;
                n_word += 1;
            }
            println!("Word: {}, frequency: {wc}", words[words_looped]);
        }

        words_looped += 1;
    }

} 
