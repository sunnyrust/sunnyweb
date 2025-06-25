use axum::routing::{get};
use crate::controller;
pub fn init() -> axum::Router {
    let website_name = std::env::var("WEBSITE_NAME").unwrap_or_else(|_| "webhotel".to_string());
    let website_name = website_name.clone();
    let website_name_root = website_name.clone();
    axum::Router::new()
        .route("/ping", get( move || async move { 
            format!("🌱🌎 I'm lying flat.🌐🌱\r\n🌱🌎 I'm locking down in webhotel.🌐🌱\r\n🌱🌎 The web project's name is {}.🌐🌱", website_name) 
        }))
        .route("/", get(move || async move { 
            format!("🌱🌎 I'm lying flat.🌐🌱\r\n🌱🌎 I'm locking down in webhotel.🌐🌱\r\n🌱🌎 The web project's name is {}.🌐🌱", website_name_root)
         }))
        .nest("/login", controller::login::router())
        .nest("/index", controller::index::router())
}