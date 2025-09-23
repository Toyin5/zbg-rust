use crate::utils::run_git;

pub fn stash(){
   run_git(&["stash", "push", "-m", "WIP"]);
}

pub fn unstash(){
    run_git(&["stash", "pop"]);
}