use std::io;
use rand::seq::SliceRandom;

fn main() { 
    loop {
        let mut factors: Vec<i64> = Vec::new();
        let mut possible_factor: i64 = 1;
        let mut factor_amount: u64 = 0;
        let mut usr_num = String::new();
        println!("Insert a number: ");
        io::stdin()
            .read_line(&mut usr_num)
            .expect("Failed to read line");
        let usr_num: i64 = match usr_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        while usr_num - possible_factor != -1 {
                if usr_num % possible_factor != 0 {
                    possible_factor += 1;
                }
                else if usr_num % possible_factor == 0 {
                    if possible_factor == 1 || possible_factor == usr_num {
                        factor_amount += 1;
                        possible_factor += 1;
                    }
                    else if possible_factor != 1 || possible_factor != usr_num {
                        factor_amount += 1;
                        factors.push(possible_factor);
                        possible_factor += 1;  
                    }
                }
                if factor_amount >= 3 {
                    break;
                }
            }
        if factor_amount == 2 {
            println!("Is Prime");
        }
        else {
            println!("Not Prime, can be divisible by {:?}", factors.choose(&mut rand::thread_rng()));
        }
    }
}