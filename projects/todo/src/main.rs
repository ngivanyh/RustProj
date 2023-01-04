use serde_json::{self, json};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut usr_input = String::new();
    println!("Welcome to todo.rs, Enter a command: ");
    io::stdin()
        .read_line(&mut usr_input)
        .expect("Failed to read line");
    let usr_cmd_vec: Vec<&str> = usr_input.split_whitespace().collect();
    if usr_cmd_vec.len() == 1 || usr_cmd_vec.len() == 0 {
        println!("Not enough commands given")
    } else {
        if usr_cmd_vec[1] == "new" {
            struct TodoInfo {
                todo_name: String,
                todo_deadline: String,
                additional_info: String,
            }
            let mut todo_info = TodoInfo {
                todo_name: "".to_string(),
                todo_deadline: "".to_string(),
                additional_info: "".to_string(),
            };
            println!("Enter Todo name: ");
            io::stdin()
                .read_line(&mut todo_info.todo_name)
                .expect("Failed to read line");
            println!("Enter Todo deadline: ");
            io::stdin()
                .read_line(&mut todo_info.todo_deadline)
                .expect("Failed to read line");
            println!("Enter additonal notes: ");
            io::stdin()
                .read_line(&mut todo_info.additional_info)
                .expect("Failed to read line");
            println!(
                "Todo Info:\n\tName: {}\tDeadline: {}\tAdditional Info: {}",
                todo_info.todo_name, todo_info.todo_deadline, todo_info.additional_info
            );
            let json_obj = json!(
                {
                    "1. Todo Name": todo_info.todo_name,
                    "2. Todo Deadline": todo_info.todo_deadline,
                    "3. Additional Notes": todo_info.additional_info,
                }
            );
            if Path::new("./todo.json").exists() {
                println!("append");
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open("./todo.json")
                    .unwrap();
                writeln!(file, "{json_obj}").expect("Cannot Append to File");
            } else if Path::new("./todo.json").exists() == false {
                println!("make new");
                let _file = File::create("./todo.json").unwrap();
                let mut file_opt = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open("./todo.json")
                    .unwrap();
                write!(file_opt, "{json_obj}\n").expect("Cannot Append to File");
            }
        }
        // desired features:
        // sort todos
        // create todos with deadline
        // add additional info to todo
        // use json file format
        // list todos
        // remind what todos have to be done that has deadline
        // ex: 2022/12/31 do something
        // auto remove todos after deadline date
        // edit todos
        // config options
    }
}
