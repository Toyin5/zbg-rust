use crate::utils::{get_current_branch, run_git};

pub fn sync(){
    let branch = get_current_branch();
    run_git(&["pull", "--ff-only", "origin", branch.as_str()]);
    println!("Synced with remote repository");
}
pub fn sync_force(){
    let branch = get_current_branch();
    run_git(&["fetch", "origin", branch.as_str()]);
    run_git(&["reset", "--hard", "origin/{}".replace("{}", branch.as_str()).as_str()]);
    println!("Synced with remote repository");
}