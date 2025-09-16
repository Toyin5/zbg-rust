use std::str;

use crate::utils::run_git;
pub fn new(branch_name: &str) {
    run_git(&["switch", "--create", branch_name]);
    println!("Switched to a new branch '{}'", branch_name);
}