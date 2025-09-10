use crate::utils::run_git;

pub fn git_add(files: &[&str]) -> std::io::Result<()> {
    let mut changes : Vec<&str> = vec!["add"];

    if files.is_empty() {
        changes.push(".");
    }

    for f in files {
        changes.push(f);
    }

    run_git(&changes);
    Ok(())
}