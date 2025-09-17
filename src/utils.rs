use std::process::Command;

pub fn run_git(args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("failed to run git command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn get_current_branch() -> String {
    let output = run_git(&["branch", "--show-current"]);
    output.trim().to_string()
}