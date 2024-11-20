use axum::routing::{get};
pub fn init() -> axum::Router {
    axum::Router::new()
        .route("/ping", get(|| async { "🌱🌎 I'm lying flat.\r\nI'm locking down in webhotel.🌐🌱" }))
}