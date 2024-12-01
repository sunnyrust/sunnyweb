pub mod lycheecli;
pub mod resources;

use lazy_static::lazy_static;
use lycheecli::config::Config;
lazy_static! {
    pub static ref CFG: Config = Config::from_file("./configs/app.toml").unwrap();
}
// import git_info moudile
include!(concat!(env!("OUT_DIR"), "/git_version.rs"));