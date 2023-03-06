use std::{env, fs};
use text_colorizer::Colorize;


fn hasNext(v: Vec<String>, current_word: i32) -> bool{
    if current_word + 1 == (v.len() - 1 as i32) {
        true
    } else {
        false
    }
}

fn main() { 
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.len() != 1 {
        eprintln!("{}", "Error: Expected 1, got more than one command line arguments".red().bold());
        std::process::exit(1);
    }
    
    let contents = fs::read_to_string(&args[0]).expect("Failed to read contents");

    let mut words: Vec<&str> = contents.split_whitespace().collect();

    words.sort();

    let mut cur_word: usize = 0;
    let mut wc = 0;
    
    println!("{:?}", words);
    
    while cur_word != words.len() - 1 {
        wc = 0;
        let mut n_word = cur_word + 1;
        if words[cur_word] == words[n_word] && n_word != words.len() - 1 && cur_word != words.len() - 1 {
            wc += 2;
            cur_word += 1;
            n_word += 1;
            while cur_word != words.len() - 1 && words[n_word] == words[cur_word] {
                cur_word += 1;
                wc += 1;
                n_word += 1;
                
            }
            println!("Word: {}, frequency: {wc}", words[cur_word]);
        }

        cur_word += 1;
    }
} 
