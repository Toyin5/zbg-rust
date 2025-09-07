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
}

fn run_git(args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("failed to run git command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_file_statuses(commit: &str) -> Vec<FileStatus> {
    let output = run_git(&["diff", "--name-status", commit]);

    output.lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let kind = parts.next()?;
            let file = parts.collect::<Vec<_>>().join(" ");
            PatchType::from_str(kind).map(|pt| FileStatus { patch_type: pt, file })
        })
        .collect()
}

fn status(commit: &str) {
    let files = get_file_statuses(commit);
    if files.is_empty() {
        println!("{}", "No changes to commit!".green());
    } else {
        for f in files {
            println!("{}  {}", f.patch_type.display(), f.file);
        }
    }
}