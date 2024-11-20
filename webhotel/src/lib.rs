//! This is documentation for the `webhotel` lib crate
//! 
//! Usage:
//! ```
//!     let web=webhotel::new();
//! ```

mod err;
pub mod config;
pub mod router;
use dotenv::dotenv;
use axum::{
    http::StatusCode,
    response::IntoResponse,
};
use std::{io};
pub async  fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}


// use tracing_subscriber::prelude::*;

pub fn new(website_name:&str)->config::Config{
    if std::env::var_os("RUST_LOG").is_none() {
        let v=format!("{}=debug",website_name);
        std::env::set_var("RUST_LOG", v);
    }
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let aaa=config::Config::from_file("./webhotel.toml").unwrap();
    // println!("{:?}\t{:?}",&aaa.web.addr,&aaa.web.version);
    //config::Config::from_env().unwrap()
    aaa
    // config::Config::from_file("./webhotel.conf").unwrap()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
