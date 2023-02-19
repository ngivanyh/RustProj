use std::io;

fn main() {
    let mut user_input = String::new();
    println!("Enter a number");
    
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: i64 = user_input.trim().parse().expect("Couldn't convert to int");
    let mut result: i64 = user_input.clone();
    
    // collatz checking
    let mut sequence_steps: u64 = 0;

    println!("You entered {user_input}\nStarting collatz sequence");
    while result != 1 {
        if result % 2 == 0 {
            result /= 2;
            sequence_steps += 1;
        } else {
            result = result * 3 + 1;
            sequence_steps += 1;
        }
    }
    println!("This collatz sequence is {sequence_steps} steps");

    // prime number checking
    println!("Starting prime number checking");
    
    if user_input == 1 {
        println!("Not prime, has 1 factor");
    }

    let mut pos_factors: i64 = 1;
    let mut factors: u64 = 0;

    while user_input > pos_factors {
        if user_input % pos_factors == 0 {
            factors += 1;
            pos_factors += 1;
        } else {
            pos_factors += 1;
        }
    }
    
    if factors > 2 {
        println!("Not prime, has {factors} factors");
    } else if factors == 2{
        println!("Is prime");
    }
}
