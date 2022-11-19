use std::io;

fn main() { 
    let mut pos_factor: i32 = 1;
    let mut how_factors: u32 = 0;
    loop {
        let mut usr_num = String::new();
        println!("Insert a number: ");
        io::stdin()
            .read_line(&mut usr_num)
            .expect("Failed to read line");
        let usr_num: i32 = match usr_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        while usr_num - pos_factor != -1 {
            if usr_num % pos_factor != 0 {
                pos_factor += 1;
            }
            if usr_num % pos_factor == 0 {
                how_factors += 1;
                pos_factor += 1;
            }
        }
        if how_factors == 2 {
            println!("Is Prime");
        }
        else {
            println!("Not Prime");
        }
    } 
}