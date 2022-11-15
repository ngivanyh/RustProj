use std::io;

fn main() {
    println!("Insert a number: ");
    let mut usr_num = String::new();
    io::stdin()
        .read_line(&mut usr_num)
        .expect("Failed to read line");
    let mut usr_num: i32 = usr_num.trim().parse().expect("Please type a number");
    let mut posFactor: i32 = 0;
    let mut howFactors: u32 = 0;
}
