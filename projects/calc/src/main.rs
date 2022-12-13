use std::io;

fn calc() {
	println!("Hello")
}
fn main() {
    io::stdin()
    	.read_line(&mut usr_choice)
    	.expect("Failed to read line");
}
