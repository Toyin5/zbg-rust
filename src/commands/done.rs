use crate::utils::{get_current_branch, run_git};

pub fn done(){
    let branch = get_current_branch();
    run_git(&["switch", "main"]);
    run_git(&["branch", "--delete", &branch, "--force"]);
}