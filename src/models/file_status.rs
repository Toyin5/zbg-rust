use super::patch_type::PatchType;

#[derive(Debug)]
pub struct FileStatus {
    pub patch_type: PatchType,
    pub file: String,
    pub insertions: usize,
    pub deletions: usize,
}
