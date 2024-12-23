use std::fs;
use std::io;
use std::fs::{ OpenOptions,File};
use std::io::{Write,ErrorKind};

#[derive(Debug,Clone)]
pub struct TemplateParams {
    pub flag: bool,
    pub source: String,
    pub target: String,
}
impl TemplateParams {

    #[allow(dead_code)]
    pub fn new(flag:bool,source:String,target:String) -> Self {
        TemplateParams {
            flag: flag,
            source: source,
            target: target,
        }
    }
}
impl Default for TemplateParams {
    fn default() -> Self {
        TemplateParams {
            flag: false,
            source: "".to_string(),
            target: "".to_string(),
        }
    }
}

#[allow(dead_code)]
/// remove directory all content
pub(crate) fn remove_dir(path: &str) -> std::io::Result<()> {
    fs::remove_dir_all(path)
}
#[allow(dead_code)]
pub(crate) fn check_file_exists(filename: &str) -> bool {
    fs::metadata(filename).is_ok()
}
#[allow(dead_code)]
/// create empty file
pub(crate) fn create_empty_file(filename: &str) -> io::Result<()> {
    match File::create(filename) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => {
                println!("File: {} is exists", filename);
                Ok(())
            },
            _ => Err(e),
        },
    }
}
#[allow(dead_code)]
/// copy file
pub(crate) fn copy_file(src: &str, dest: String) -> io::Result<()> {
    fs::copy(src, &dest)?;
    Ok(())
}
/// create file
pub(crate)  fn create_file_from_str(path:String,body: &[u8],msg:String)-> std::io::Result<()>{
    let mut buffer = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(&path).unwrap();

    buffer.write_all(body)?;
    println!("{}",msg);
    buffer.flush()?;

    Ok(())
}
/// create file from template
pub(crate) fn create_file_from_template(path:String,template:String,tp:TemplateParams)-> std::io::Result<()>{
    let mut buffer = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(&path).unwrap();
    let mut content = fs::read_to_string(&template).expect("Failed to read file");
    if tp.flag {
        content = content.replace( &tp.source, &tp.target);
    }
    buffer.write(content.as_bytes())?;
    Ok(())
}