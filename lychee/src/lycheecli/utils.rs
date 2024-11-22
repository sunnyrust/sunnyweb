use std::fs;

/// remove directory all content
pub(crate) fn remove_dir(path: &str) -> std::io::Result<()> {
    fs::remove_dir_all(path)
}