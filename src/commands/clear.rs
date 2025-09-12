use crate::utils::run_git;

pub fn clear(){
    let output = run_git(&["reset", "--hard"]);
    println!("{}", output);
}