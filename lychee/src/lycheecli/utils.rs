use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
/// remove directory all content
pub(crate) fn remove_dir(path: &str) -> std::io::Result<()> {
    fs::remove_dir_all(path)
}
pub(crate) fn check_file_exists(filename: &str) -> bool {
    fs::metadata(filename).is_ok()
}
pub(crate) fn create_empty_file(filename: &str) -> io::Result<()> {
    match File::create(filename) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => {
                println!("文件已存在: {}", filename);
                Ok(())
            },
            _ => Err(e),
        },
    }
}