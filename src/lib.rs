use std::{env, process::exit};

use crate::commands::status::status;
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
    } else {
        println!("{} command not supported yet", args[1]);
    }
}
