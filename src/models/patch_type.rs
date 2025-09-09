use colored::Colorize;

#[derive(Debug, PartialEq, Eq)]
pub enum PatchType {
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
    pub fn from_str(s: &str) -> Option<Self> {
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
            _ => None,
        }
    }

    pub fn display(&self) -> String {
        match self {
            Self::Added => "added".green().bold().to_string(),
            Self::Copied => "copied".magenta().bold().to_string(),
            Self::Deleted => "deleted".red().bold().to_string(),
            Self::Modified => "modified".blue().bold().to_string(),
            Self::Renamed => "renamed".yellow().bold().to_string(),
            Self::TypeChanged => "type-changed".cyan().bold().to_string(),
            Self::Unmerged => "unmerged".bold().to_string(),
            Self::Unknown => "unknown".bold().to_string(),
            Self::BrokenPairing => "broken".bold().to_string()
        }
    }
}
