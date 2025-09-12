use std::{env, process::exit};

use crate::commands::{add::git_add, clear::clear, log::log, status::status};
pub mod commands;
pub mod models;
pub mod utils;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No command provided");
        exit(0);
    }
    if args[1].eq("status") {
        status("HEAD");
    } else if args[1].eq("add") {
        let files: Vec<&str> = if args.len() > 2 {
            args[2..].iter().map(|s| s.as_str()).collect()
        } else {
            vec![]
        };
        match git_add(&files) {
            Ok(_) => {},
            Err(e) => eprintln!("Error adding files: {}", e),
        }
    }else if args[1].eq("log"){
        let limit = if args.len() > 2 {
            args[2].parse::<usize>().unwrap_or(5)
        } else {
            5
        };
        log(limit);
    } else if args[1].eq("clear"){
        clear();
    }
    else {
        println!("{} command not supported yet", args[1]);
    }
}
