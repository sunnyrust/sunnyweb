//! This is documentation for the `webhotel` lib crate
//! 
//! Usage:
//! ```
//!     let web=webhotel::new();
//! ```
pub mod config;
pub mod controller;
pub mod dbstate;
pub mod model;
pub mod err;
pub mod router;
pub mod utils;
use dotenv::dotenv;
use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
    middleware::Next, 
    extract::Request, 
    response::Response
};
use std::{io};
use tera::Tera;
use std::sync::{Arc,Mutex};
use dbstate::DbState;

use std::collections::HashMap;
use serde_json::Value;
use lazy_static::lazy_static;

pub const CONFIG_PATH: &str = "./configs/webhotel.toml";

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<String, Value> = {
        let mut translations = HashMap::new();
        
        // // load English translations
        // if let Ok(content) = std::fs::read_to_string("configs/locales/zh-CN.toml") {
        //     if let Ok(parsed) = toml::from_str(&content) {
        //         translations.insert("zh-CN".to_string(), parsed);
        //     }
        // }
        // // load English translations
        // if let Ok(content) = std::fs::read_to_string("configs/locales/en-US.toml") {
        //     if let Ok(parsed) = toml::from_str(&content) {
        //         translations.insert("en-US".to_string(), parsed);
        //     }
        // }
        if let Ok(config) = config::Config::from_file(CONFIG_PATH) {
            for locale in &config.langconf.supported {
                let file_path = format!("configs/locales/{}.toml", locale);
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    if let Ok(parsed) = toml::from_str(&content) {
                        translations.insert(locale.clone(), parsed);
                    }
                }
            }
        }
        translations
    };
}
pub fn get_translation(lang: &str) -> Option<&Value> {
    TRANSLATIONS.get(lang)
}
#[derive(Clone)]
pub struct BaseController {
    pub uri: Uri,
    pub app_version: String,
}

impl BaseController {
    pub fn new(uri: Uri, app_version: String) -> Self {
        Self { uri, app_version }
    }

    pub fn log_request(&self) {
        println!("Request URI: {}\r\nApp Version: {}", self.uri.path(), self.app_version);
    }
}



pub async  fn base_controller_middleware(request: Request, next: Next) -> Response {
    let uri = request.uri().clone();
    let app_version = std::env::var("WEBSITE_VERSION").unwrap_or_default();
    let base_controller = BaseController::new(uri,app_version);
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
    // config::Config::from_file("./configs/webhotel.toml").unwrap()
    match config::Config::from_file(CONFIG_PATH) {
        Ok(c) => c,
        Err(e) => {
            panic!("Failed to load config file at {}: {}", CONFIG_PATH, e);
        }
    }
}


// app state
pub struct AppState {
    pub tera: Tera,
    pub path_segments: Mutex<Vec<String>>,
    pub db_state: DbState,
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
            db_state: DbState {
                conn: self.db_state.conn.clone(), // Assuming PgPool supports Clone
                redis_conn: self.db_state.redis_conn.clone(), // Assuming Client supports Clone
            },
        }
    }
}

pub use err::{AppError, AppErrorType};
pub type Result<T> = std::result::Result<T, crate::AppError>;
// #[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
