use colored::Colorize;

use crate::models::file_status::FileStatus;
use crate::models::patch_type::PatchType;
use crate::utils::run_git;

fn get_file_statuses(commit: &str) -> Vec<(String, PatchType)> {
    let output = run_git(&["diff", "--name-status", commit]);
    output
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let kind = parts.next()?;
            let file = parts.collect::<Vec<_>>().join(" ");
            PatchType::from_str(kind).map(|pt| (file, pt))
        })
        .collect()
}

fn get_file_stats(commit: &str) -> Vec<(String, usize, usize)> {
    let output = run_git(&["diff", "--stat", commit]);
    let mut stats = Vec::new();

    for line in output.lines() {
        if let Some((file, rest)) = line.split_once('|') {
            let file = file.trim().to_string();
            let insertions = rest.matches('+').count();
            let deletions = rest.matches('-').count();
            stats.push((file, insertions, deletions));
        }
    }
    stats
}

/// Merge statuses + stats
fn collect_status(commit: &str) -> Vec<FileStatus> {
    let statuses = get_file_statuses(commit);
    let stats = get_file_stats(commit);

    statuses
        .into_iter()
        .map(|(file, patch_type)| {
            let (insertions, deletions) = stats
                .iter()
                .find(|(f, _, _)| *f == file)
                .map(|(_, i, d)| (*i, *d))
                .unwrap_or((0, 0));
            FileStatus {
                patch_type,
                file,
                insertions,
                deletions,
            }
        })
        .collect()
}

/// Render insertions/deletions as colored blocks
fn render_changes(ins: usize, del: usize) -> String {
    let ins_str = "■".repeat(ins).green().to_string();
    let del_str = "■".repeat(del).red().to_string();
    let changes_count = ins + del;
    format!("{} {}{}", changes_count,ins_str, del_str)
}


pub fn status(commit: &str) {
    let files = collect_status(commit);
    if files.is_empty() {
        println!("{}", "No changes to commit!".green());
    } else {
        for f in files {
            println!(
                "{}  {:<20} | {}",
                f.patch_type.display(),
                f.file,
                render_changes(f.insertions, f.deletions)
            );
        }
    }
}