use colored::Colorize;

use crate::models::file_status::{FileStatus};
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

/// Parse `git diff --no-index /dev/null file` for untracked file
fn get_untracked_diff_stat(file: &str) -> Option<(usize, usize)> {
    let output = run_git(&["diff", "--no-index", "--stat", "/dev/null", file]);

    output
        .lines()
        .find_map(|line| {
            if let Some(idx) = line.find('|') {
                let parts: Vec<&str> = line[idx + 1..].split_whitespace().collect();
                if let Some(signs) = parts.last() {
                    let pluses = signs.chars().filter(|&c| c == '+').count();
                    let minuses = signs.chars().filter(|&c| c == '-').count();
                    return Some((pluses, minuses));
                }
            }
            None
        })
}

fn get_untracked_files() -> Vec<FileStatus> {
    let output = run_git(&["ls-files", "--others", "--exclude-standard"]);
    output
        .lines()
        .map(|file| {
            let (insertions, deletions) = get_untracked_diff_stat(file).unwrap_or((0, 0));
            FileStatus {
            patch_type: PatchType::Added,
            file: file.to_string(),
            insertions,
            deletions,
        }
    })
        .collect()
}

/// Merge statuses + stats
fn collect_status(commit: &str) -> Vec<FileStatus> {
    let statuses = get_file_statuses(commit);
    let stats = get_file_stats(commit);

    let mut untracked = get_untracked_files();

    let mut all_status: Vec<FileStatus> = statuses
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
        .collect();
    all_status.append(&mut untracked);
    all_status
}

/// Render insertions/deletions as colored blocks
fn render_changes(ins: usize, del: usize) -> String {
    let ins_str = "■".repeat(ins).green().to_string();
    let del_str = "■".repeat(del).red().to_string();
    let changes_count = ins + del;
    format!("\t{} {}{}", changes_count, ins_str, del_str)
}

pub fn status(commit: &str) {
    let files = collect_status(commit);

    if files.is_empty() {
        println!("{}", "No changes to commit!".green());
        return;
    }

    // 1. Measure max widths
    let max_type_len = files
        .iter()
        .map(|f| f.patch_type.display().len())
        .max()
        .unwrap_or(0);

    let max_file_len = files.iter().map(|f| f.file.len()).max().unwrap_or(0);

    // 2. Print aligned columns
    for f in files {
        println!(
            "{:<type_width$}  {:<file_width$}  |{}",
            f.patch_type.display(),
            f.file,
            render_changes(f.insertions, f.deletions),
            type_width = max_type_len,
            file_width = max_file_len
        );
    }
}
