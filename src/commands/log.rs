use colored::Colorize;

use crate::utils::run_git;

pub fn log(limit:usize){
  let args = &[
        "log",
        &format!("-n{}", limit),
        "--decorate=short",
        "--pretty=format:%h|%d|%an|%ar|%s",
    ];

    let output = run_git(args);
     for line in output.lines() {
        let parts: Vec<&str> = line.splitn(5, '|').collect();
        if parts.len() == 5 {
            let hash = parts[0].green().bold();
            let decorations = parts[1].trim();
            let author = parts[2].white();
            let date = parts[3].white();
            let subject = parts[4].cyan().italic();

            let author_text = "Author".blue().bold();
            let date_text = "Date".blue().bold();

            if decorations.is_empty() {
                 println!(
                    "{}: {}  \n\t {}: {} \n\t   {}: {}",
                    hash,
                    subject,
                    author_text,
                    author,
                    date_text,
                    date
                );
            } else {
                println!(
                    "{}: {}  {} \n\t {}: {} \n\t   {}: {}",
                    hash,
                    subject,
                    decorations.bright_yellow(),
                    author_text,
                    author,
                    date_text,
                    date
                );
            }
        }
    }
}