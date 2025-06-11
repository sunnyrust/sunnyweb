use axum::routing::{get};
use crate::controller;
pub fn init() -> axum::Router {
    axum::Router::new()
        .route("/ping", get(|| async { "🌱🌎 I'm lying flat.\r\nI'm locking down in webhotel.🌐🌱" }))
        .route("/", get(|| async { "🌱🌎 I'm lying flat.\r\nI'm locking down in webhotel.🌐🌱" }))
        .nest("/login", controller::login::login_router())
}