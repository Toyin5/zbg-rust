use crate::utils::run_git;

pub fn clear(){
   run_git(&["reset", "--hard"]);
}