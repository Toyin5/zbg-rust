use crate::utils::run_git;

pub fn switch(){
    run_git(&["switch", "main"]);
    run_git(&["pull", "--ff-only", "--prune"]);
}