use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use chrono::Local;
fn main() {
    // get Git version
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("failed to execute git command");

    let git_version = String::from_utf8_lossy(&output.stdout);

    // insert git version to a file
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = format!("{}/git_version.rs", out_dir);
    // get current time
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let content = format!("pub const GIT_VERSION: &str = \"1.0--{}\ngit:{}\";",timestamp, git_version);

    let mut file = File::create(&dest_path).expect("failed to create file");
    file.write_all(content.as_bytes()).expect("failed to write to file");

    println!("cargo:rustc-env=GIT_VERSION={}", git_version); // pass git version to ENV
}