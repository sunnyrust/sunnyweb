use std::fs;
use std::io;
use std::fs::{ OpenOptions,File};
use std::io::{Write,ErrorKind};
use argon2::{self, Config, Variant, Version};
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


/// Password hashing utility using Argon2
pub struct PasswordHasher {
    salt:String, // Example salt, should be generated securely
    config: Config<'static>,
}
/// Implementing PasswordHasher  struct
impl PasswordHasher {
    /// Creates a new PasswordHasher with default configuration
    pub fn new() -> Self {
        Self {
            salt: "randomsalt".into(),
            config: Config {
                variant: Variant::Argon2i,
                version: Version::Version13,
                mem_cost: 65536,
                time_cost: 10,
                lanes: 4,
                secret: &[],
                ad: &[],
                hash_length: 32
            },
        }
    }
    /// Creates a new PasswordHasher with a custom salt
    pub fn hash(&self, password: &str) -> String {
        let hash = argon2::hash_encoded(password.as_bytes(), &self.salt.as_bytes(), &self.config).unwrap();
        hash
    }
    /// Verifies a password against an encoded hash
    pub fn verify(&self, password: &str, encoded_hash: &str) -> bool {
        argon2::verify_encoded(encoded_hash, password.as_bytes()).unwrap_or(false)
    }
}