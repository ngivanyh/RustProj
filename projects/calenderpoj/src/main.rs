use chrono::prelude::*;
use std::io;

fn cal(y: i32, mo: u32) {
    // debug
    println!("gotten to func y{},mo{}", y, mo);
    // let dt = Local.with_ymd_and_hms(y, mo, 0, 0, 0, 0);
    // identify how many times cal has to loop
    if mo == 1 || mo == 3 || mo == 5 || mo == 7 || mo == 8 || mo == 10 || mo == 12 {
        println!("These months have 31 days");
    } else if mo == 4 || mo == 6 || mo == 9 || mo == 11 {
        println!("These months have 30 days");
    } else if mo == 2 {
        if y % 4 == 0 {
            if y % 100 == 0 {
                if y % 400 == 0 {
                    println!("This month has 29 days");
                } else {
                    println!("This month has 28 days");
                }
            } else {
                println!("This month has 29 days");
            }
        } else {
            println!("This month has 28 days")
        }
    } else {
        println!("Incorrect month");
    }
}
fn date(cmd: String) {
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
        // declare needed var
        let mut cal_year = String::new();
        let mut cal_month = String::new();
        // specify year
        println!("Specify year:");
        io::stdin()
            .read_line(&mut cal_year)
            .expect("Failed to read line");
        // debug
        println!("{}", cal_year);
        // specify month
        println!("Specify month:");
        io::stdin()
            .read_line(&mut cal_month)
            .expect("Failed to read line");
        // debug
        println!("{}", cal_month);
        // type conversion
        let cal_year: i32 = cal_year
            .trim()
            .parse()
            .expect("Couldn't convert string to int");
        let cal_month: u32 = cal_month
            .trim()
            .parse()
            .expect("Couldn't convert string to int");
        // gives ownership to cal
        cal(cal_year, cal_month);
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
    date(usr_in);
}
