use std::process::Command;

pub fn run_git(args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("failed to run git command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
