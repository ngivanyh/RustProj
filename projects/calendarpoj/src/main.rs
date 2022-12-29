use chrono::prelude::*;
use std::io;

fn cal_maker(mo_days: u32, mo: u32, y: i32) {
    println!("hello this month has{}", mo_days);
    let mut cnt: u32 = 1;
    println!("{}", format!("{:^27}", format!("{} {}", mo, y)));
    println!("Sun Mon Tue Wed Thu Fri Sat");
    while mo_days >= cnt {
        let mut week: Vec<u32> = Vec::new();
        while week.len() <= 6 {
            if cnt == 1 {
                let dt = Local.with_ymd_and_hms(y, mo, 1, 0, 0, 0).unwrap();
                let fmt_dt = format!("{}", dt.format("%a"));
                let sliced_dt = &fmt_dt[0..];
                if sliced_dt == "Sun" {
                    week.push(cnt);
                } else if sliced_dt == "Mon" {
                    week.push(0);
                    week.push(cnt);
                } else if sliced_dt == "Tue" {
                    week.push(0);
                    week.push(0);
                    week.push(cnt);
                } else if sliced_dt == "Wed" {
                    week.push(0);
                    week.push(0);
                    week.push(0);
                    week.push(cnt);
                } else if sliced_dt == "Thu" {
                    week.push(0);
                    week.push(0);
                    week.push(0);
                    week.push(0);
                    week.push(cnt);
                } else if sliced_dt == "Fri" {
                    week.push(0);
                    week.push(0);
                    week.push(0);
                    week.push(0);
                    week.push(0);
                    week.push(cnt);
                } else if sliced_dt == "Sat" {
                    week.push(0);
                    week.push(0);
                    week.push(0);
                    week.push(0);
                    week.push(0);
                    week.push(0);
                    week.push(cnt);
                }
            } else if cnt == mo_days {
                if week.len() != 7 {
                    week.push(cnt);
                    while week.len() != 7 {
                        week.push(0);
                    }
                } else if week.len() == 7 {
                    continue;
                }
            } else {
                week.push(cnt);
            }
            cnt += 1;
        }
        for (pos, _e) in week.iter().enumerate() {
            if week[pos] != 0 {
                print!("{:>3}", format!("{}", week[pos]));
            } else {
                print!("{:>3}", format!(" "));
            }
            print!(" ");
        }
        println!("");
    }
}
fn cal(y: i32, mo: u32) {
    // debug
    println!("gotten to func y{},mo{}", y, mo);
    // identify how many times cal has to loop
    if mo == 1 || mo == 3 || mo == 5 || mo == 7 || mo == 8 || mo == 10 || mo == 12 {
        println!("These months have 31 days");
        let days_in_mo: u32 = 31;
        cal_maker(days_in_mo, mo, y);
    } else if mo == 4 || mo == 6 || mo == 9 || mo == 11 {
        println!("These months have 30 days");
        let days_in_mo: u32 = 30;
        cal_maker(days_in_mo, mo, y);
    } else if mo == 2 {
        if y % 4 == 0 {
            if y % 100 == 0 {
                if y % 400 == 0 {
                    println!("This month has 29 days");
                    let days_in_mo: u32 = 29;
                    cal_maker(days_in_mo, mo, y);
                } else {
                    println!("This month has 28 days");
                    let days_in_mo: u32 = 28;
                    cal_maker(days_in_mo, mo, y);
                }
            } else {
                println!("This month has 29 days");
                let days_in_mo: u32 = 29;
                cal_maker(days_in_mo, mo, y);
            }
        } else {
            println!("This month has 28 days");
            let days_in_mo: u32 = 28;
            cal_maker(days_in_mo, mo, y);
        }
    } else {
        println!("Incorrect month");
    }
}
fn date(cmd: String) {
    // declare vars
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
