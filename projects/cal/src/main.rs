use std::io;
use chrono;

fn get_date(cmd: String) {
	println!("{:?}", chrono::offset::Local::now());
	println!("{}", cmd)
}
fn main() {
	let mut usr_cmd = String::new();
	io::stdin()
    	.read_line(&mut usr_cmd)
    	.expect("Failed to read line");
	get_date(usr_cmd);
}

