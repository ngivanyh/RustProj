use chrono::prelude::*;
use std::io;

fn cal_maker(mo_days: u32, mo: u32, y: i32) {
    println!("hello this month has{}", mo_days);
    // declare var
    let mut sun: Vec<u32> = Vec::new();
    let mut mon: Vec<u32> = Vec::new();
    let mut tue: Vec<u32> = Vec::new();
    let mut wed: Vec<u32> = Vec::new();
    let mut thur: Vec<u32> = Vec::new();
    let mut fri: Vec<u32> = Vec::new();
    let mut sat: Vec<u32> = Vec::new();
    let mut cnt: u32 = 1;
    println!("hello cars");
    while mo_days >= cnt {
        println!("hello loop");
        let dt = Local.with_ymd_and_hms(y, mo, cnt, 0, 0, 0).unwrap();
        let format_dt = format!("{}", dt.format("%a"));
        println!("{}", format_dt.len());
        let sliced_format_dt = &format_dt[0..];
        if sliced_format_dt == "Sun" {
            sun.push(cnt);
            println!("hello sun");
            // cnt += 1;
        } else if sliced_format_dt == "Mon" {
            mon.push(cnt);
            println!("hello mon");
            // cnt += 1;
        } else if sliced_format_dt == "Tue" {
            tue.push(cnt);
            println!("hello tue");
            // cnt += 1;
        } else if sliced_format_dt == "Wed" {
            wed.push(cnt);
            println!("hello wed");
            // cnt += 1;
        } else if sliced_format_dt == "Thu" {
            thur.push(cnt);
            println!("hello thur");
            // cnt += 1;
        } else if sliced_format_dt == "Fri" {
            fri.push(cnt);
            println!("hello fri");
            // cnt += 1;
        } else if sliced_format_dt == "Sat" {
            sat.push(cnt);
            println!("hello sat");
            // cnt += 1;
        }
        cnt += 1;
    }
    cnt = 1;
    println!("sun{sun:?}mon{mon:?}tue{tue:?}wed{wed:?}thur{thur:?}fri{fri:?}sat{sat:?}");
    println!("{}", format!("{:^27}", format!("{} {}", mo, y)));
    println!("Sun Mon Tue Wed Thu Fri Sat");
    while mo_days >= cnt {
        let mut week: Vec<u32> = Vec::new();
        while week.len() <= 6 {
            // let first_dt = Local.with_ymd_and_hms(y, mo, 1, 0, 0, 0).unwrap();
            // let first_fmt_dt = format!("{}", first_dt.format("%a"));
            // let sliced_first_dt = &first_fmt_dt[0..];
            week.push(cnt);
            cnt += 1;
        }
        cnt += 1;
        // println!(
        //     "  {:?}   {:?}   {:?}   {:?}   {:?}   {:?}   {:?}",
        //     week[0], week[1], week[2], week[3], week[4], week[5], week[6],
        // );
        println!(
            "{} {} {} {} {} {} {}",
            format!("{:>3}", format!("{}", week[0])),
            format!("{:>3}", format!("{}", week[1])),
            format!("{:>3}", format!("{}", week[2])),
            format!("{:>3}", format!("{}", week[3])),
            format!("{:>3}", format!("{}", week[4])),
            format!("{:>3}", format!("{}", week[5])),
            format!("{:>3}", format!("{}", week[6])),
        )
    }

    //          12 2022
    // Sun Mon Tue Wed Thu Fri Sat
    //                   1   2   3
    //   4   5   6   7   8   9   10
    //   11  12  13  14  15  16  17
    //   18  19  20  21  22  23  24
    //   25  26  27  28  29  30  31
    // how to acheive goal:
    // make a array that has a list of the dates in order
    // ex: 12/1 is on friday = [" ", " ", ... "1"]
    // you add spaces in front of first day if first day is not on sunday
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
