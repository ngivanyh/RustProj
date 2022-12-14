use std::io;

fn cal(cmd: String) {
    println!("{}", cmd);
}
fn main() {
    let mut usr_in = String::new();
    io::stdin()
        .read_line(&mut usr_in)
        .expect("Failed to read line");
    cal(usr_in);
}
