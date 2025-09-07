use std::{env, process::{exit, Command}};
use colored::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        exit(1);
    }
    if args[1].eq("status") {
        status(&"HEAD");
    }else{
        println!("{} command not supported yet", args[1]);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PatchType {
    Added,
    Copied,
    Deleted,
    Modified,
    Renamed,
    TypeChanged,
    Unmerged,
    Unknown,
    BrokenPairing,
}

impl PatchType {
    fn from_str(s: &str) -> Option<Self> {
        match s.chars().next()? {
            'A' => Some(Self::Added),
            'C' => Some(Self::Copied),
            'D' => Some(Self::Deleted),
            'M' => Some(Self::Modified),
            'R' => Some(Self::Renamed),
            'T' => Some(Self::TypeChanged),
            'U' => Some(Self::Unmerged),
            'X' => Some(Self::Unknown),
            'B' => Some(Self::BrokenPairing),
            _   => None,
        }
    }

    fn display(&self) -> String {
        match self {
            PatchType::Added => "added".green().bold().to_string(),
            PatchType::Copied => "copied".magenta().bold().to_string(),
            PatchType::Deleted => "deleted".red().bold().to_string(),
            PatchType::Modified => "modified".blue().bold().to_string(),
            PatchType::Renamed => "renamed".yellow().bold().to_string(),
            PatchType::TypeChanged => "type-changed".cyan().bold().to_string(),
            PatchType::Unmerged => "unmerged".bold().to_string(),
            PatchType::Unknown => "unknown".bold().to_string(),
            PatchType::BrokenPairing => "broken".bold().to_string(),
        }
    }
}

#[derive(Debug)]
struct FileStatus {
    patch_type: PatchType,
    file: String,
    insertions: usize,
    deletions: usize,
}

fn run_git(args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("failed to run git command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Parse `git diff --name-status` to get patch types
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

// Parse `git diff --stat` to extract insertions/deletions
fn get_file_stats(commit: &str) -> Vec<(String, usize, usize)> {
    let output = run_git(&["diff", "--stat", commit]);
    let mut stats = Vec::new();

    for line in output.lines() {
        // typical line: " src/lib.rs | 10 ++++++++++"
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
    format!("{}{}", ins_str, del_str)
}


fn status(commit: &str) {
    let files = collect_status(commit);
    if files.is_empty() {
        println!("{}", "No changes to commit!".green());
    } else {
        for f in files {
            println!(
                "{}  {:<20} {}",
                f.patch_type.display(),
                f.file,
                render_changes(f.insertions, f.deletions)
            );
        }
    }
}
