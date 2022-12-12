use std::io;

fn calc() {
	println!("Hello")
}
fn main() {
	let mut usr_choice = String::new();
    println!("Select your option:\n (add/subtract/multiply/divide)");
    io::stdin()
    	.read_line(&mut usr_choice)
    	.expect("Failed to read line");
}
