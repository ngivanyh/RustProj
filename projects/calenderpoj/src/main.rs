use std::io;

fn main() {
    let mut usr_in = String::new();
    io::stdin()
	.read_line(&mut usr_in)
	.expect("Failed to read line");
    println!("{}", usr_in);
}
