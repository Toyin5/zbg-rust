use std::{env, process::exit};

use crate::commands::{add::git_add, clear::clear, commit::commit, log::log, new::new, status::status, sync::{sync, sync_force}, tag::tag};
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
    else if args[1].eq("commit"){
        if args.len() < 3 {
            eprintln!("Commit message is required");
            exit(0);
        }
        let message = &args[2];
        commit(message);
    }
    else if args[1].eq("new"){
        if args.len() < 3 {
            eprintln!("Branch name is required");
            exit(0);
        }
        let branch_name = &args[2];
        new(branch_name);
    }
    else if args[1].eq("sync"){
        let flag = if args.len() > 2 {
            args[2].as_str()
        } else {
            ""
        };
        if flag == "--force" || flag == "-f" {
            sync_force();
        } else {
            sync();
        }
    }
    else if args[1].eq("tag"){
         if args.len() < 3 {
            eprintln!("Tag name is required");
            exit(0);
        }
        let desc = if args.len() > 2{
            args[2].as_str()
        }else{
            ""
        };
        tag(desc);
    }
    else {
        println!("{} command not supported yet", args[1]);
    }
}
