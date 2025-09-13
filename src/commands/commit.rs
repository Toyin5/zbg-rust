use crate::utils::run_git;
pub fn commit(message: &str) {
    run_git(&["add", "."]);
    run_git(&["commit", "-m", message]);
}