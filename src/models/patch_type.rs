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
            _   => None,
        }
    }

    pub fn display(&self) -> String {
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