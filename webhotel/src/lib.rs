//! This is documentation for the `webhotel` lib crate
//! 
//! Usage:
//! ```
//!     let web=webhotel::new();
//! ```

mod err;
pub mod config;
pub mod router;
pub mod controller;
use dotenv::dotenv;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    middleware::Next, 
    extract::Request, 
    response::Response
};
use std::{io};
use tera::Tera;
use std::sync::{Arc,Mutex};

use axum::http::Uri;
#[derive(Clone)]
pub struct BaseController {
    pub uri: Uri,
}

impl BaseController {
    pub fn new(uri: Uri) -> Self {
        Self { uri }
    }

    pub fn log_request(&self) {
        println!("Request URI: {}", self.uri.path());
    }
}



pub async  fn base_controller_middleware(request: Request, next: Next) -> Response {
    let uri = request.uri().clone();
    let base_controller = BaseController::new(uri);

    // 存储到请求扩展
    let mut request = request;
    request.extensions_mut().insert(base_controller);

    next.run(request).await
}


// use tokio::sync::{Arc,Mutex};
pub async  fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}


// use tracing_subscriber::prelude::*;

pub fn new(website_name:&str)->config::Config{
    if std::env::var_os("RUST_LOG").is_none() {
        let v=format!("{}=debug",website_name);
        std::env::set_var("RUST_LOG", v);
        //std::env::set_var("WEBSITE_NAME", website_name);
    }

    dotenv().ok();
    config::Config::from_file("./webhotel.toml").unwrap()
}


// app state
pub struct AppState {
    pub tera: Tera,
    pub path_segments: Mutex<Vec<String>>,
}
// impl Clone for AppState {
//     fn clone(&self) -> Self {
//         AppState {
//             tera: self.tera.clone(),
//             path_segments: Mutex::new(self.path_segments.blocking_lock().clone()), // Use blocking_lock() in Clone impl
//         }
//     }
// }
impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            tera: self.tera.clone(), // Tera supports Clone
            path_segments: Mutex::new(self.path_segments.lock().unwrap().clone()), // Lock and clone inner Vec<String>
        }
    }
}
// #[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
