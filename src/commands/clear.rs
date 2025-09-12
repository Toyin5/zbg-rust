use crate::utils::run_git;

pub fn clear(){
    let _ = run_git(&["reset", "--hard"]);
}