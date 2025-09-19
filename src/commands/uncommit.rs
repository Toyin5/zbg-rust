use crate::utils::run_git;

pub fn uncommit(){
    run_git(&["reset", "HEAD~1"]);
}