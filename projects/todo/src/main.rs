use std::io;

fn todo_manage(input: Vec<&str>) {
    struct Todo {
        todo_name: &'static str,
        todo_deadline: &'static str,
        additional_info: &'static str,
    }
    if input[1] == "-new" {
        let mut todo = Todo {
            todo_name: input[2],
            todo_deadline: "hello",
            additional_info: "hi"
        }
    }
}
fn main() {
    let mut usr_input = String::new();
    println!("Welcome to todo.rs, Enter a command: ");
    io::stdin()
        .read_line(&mut usr_input)
        .expect("Failed to read line");
    let usr_cmd_vec: Vec<&str> = usr_input.split_whitespace().collect();
    if usr_cmd_vec.len() == 1 || usr_cmd_vec.len() == 0 {
        println!("Not enough arguments given");
    } else {
        todo_manage(usr_cmd_vec);
    }
    // desired features:
    // sort todos
    // create todos with deadline
    // add additional info to todo
    // simple syntax:
    // ex: todo -new TODO_1 -deadline 2023/1/1 -add_info https://google.com, https://youtube.com
    // use json file format
    // list todos
    // remind what todos have to be done that has deadline
    // ex: 2022/12/31 do something
    // auto remove todos after deadline date
}
