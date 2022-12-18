use chrono::prelude::*;
use std::io;

fn cal(cmd: String) {
    // declare variable shortcuts
    let time = Local::now();
    let year = time.year();
    let month = time.month();
    let day = time.day();
    let weekday = time.weekday();
    // debug
    println!("{}", cmd);
    // commands
    if cmd == "cal\n" {
        let mut condition = true;
        while condition == true {
            
        }
        // get date and weekday of current month
        // iterate through ex: 12/1 is monday -> 12/2 is tuesday
        // stop until last day of month
    } else if cmd == "now\n" {
        println!(
            "{}-{}-{},{} {}:{}",
            year,
            month,
            day,
            weekday,
            time.hour(),
            time.minute()
        ); // get current time year-month-day,weekday hour:minute
    } else if cmd == "year\n" {
        println!("{}", year); // get year
    } else if cmd == "month\n" {
        println!("{}", month); // get month
    } else if cmd == "day\n" {
        println!("{}", day); // get day of month
    } else if cmd == "weekday\n" {
        println!("{}", weekday); // get weekday
    } else {
        println!("Not a func")
    }
}
fn main() {
    let mut usr_in = String::new();
    io::stdin()
        .read_line(&mut usr_in)
        .expect("Failed to read line");
    cal(usr_in);
}
