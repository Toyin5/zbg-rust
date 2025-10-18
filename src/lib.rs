use clap::{Parser, Subcommand};
use crate::commands::{add::git_add, clear::clear, commit::commit, done::done, log::log, new::new, push::push, rebase::rebase, stash::{stash, unstash}, status::status, switch::switch, sync::{sync, sync_force}, tag::tag, uncommit::uncommit};

pub mod commands;
pub mod models;
pub mod utils;

#[derive(Parser)]
#[command(name = "zbg-rust")]
#[command(about = "A Rust-based command-line tool for git commands", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show git status
    Status,

    /// Add files to staging area
    Add {
        /// Files to add (empty for all)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        files: Vec<String>,
    },

    /// Show git log
    Log {
        /// Number of commits to show
        #[arg(default_value = "5")]
        limit: usize,
    },

    /// Clear the repository
    Clear,

    /// Create a commit
    Commit {
        /// Commit message
        message: String,
    },

    /// Create a new branch
    New {
        /// Branch name
        branch_name: String,
    },

    /// Sync with remote
    Sync {
        /// Force sync
        #[arg(short, long)]
        force: bool,
    },

    /// Create a tag
    Tag {
        /// Tag description
        desc: String,
    },

    /// Undo the last commit
    Uncommit,

    /// Push changes
    Push,

    /// Mark as done
    Done,

    /// Stash changes
    Stash,

    /// Unstash changes
    Unstash,

    /// Switch branch
    Switch,

    /// Rebase current branch
    Rebase,
}

pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            status("HEAD");
        }
        Commands::Add { files } => {
            let files_refs: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
            match git_add(&files_refs) {
                Ok(_) => {},
                Err(e) => eprintln!("Error adding files: {}", e),
            }
        }
        Commands::Log { limit } => {
            log(limit);
        }
        Commands::Clear => {
            clear();
        }
        Commands::Commit { message } => {
            commit(&message);
        }
        Commands::New { branch_name } => {
            new(&branch_name);
        }
        Commands::Sync { force } => {
            if force {
                sync_force();
            } else {
                sync();
            }
        }
        Commands::Tag { desc } => {
            tag(&desc);
        }
        Commands::Uncommit => {
            uncommit();
        }
        Commands::Push => {
            push();
        }
        Commands::Done => {
            done();
        }
        Commands::Stash => {
            stash();
        }
        Commands::Unstash => {
            unstash();
        }
        Commands::Switch => {
            switch();
        }
        Commands::Rebase => {
            rebase();
        }
    }
}