use std::io;

fn main() {
    println!("Insert a number: ");
    let mut usr_num = String::new();
    io::stdin()
        .read_line(&mut usr_num)
        .expect("Failed to read line");
    println!("You typed {usr_num}")
}
