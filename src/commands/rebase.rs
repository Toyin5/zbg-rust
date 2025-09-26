use crate::utils::run_git;

pub fn rebase(){
    run_git(&["fetch", "origin", "main"]);
    run_git(&["rebase", "origin/main"]);
}
