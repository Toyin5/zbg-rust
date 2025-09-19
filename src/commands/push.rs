use crate::utils::{get_current_branch, run_git};

pub fn push(){
    let branch = get_current_branch();
    run_git(&["push","--set-upstream", "origin", branch.as_str()]);
}