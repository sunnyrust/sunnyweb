//login controller
use axum::{response::Html, routing::get, Router, Extension};
use tera::{Tera, Context};
use crate::AppState;
use std::sync::Arc;
use crate::controller::get_app_state;
pub fn login_router() -> Router {
    Router::new()
        .route("/login", get(render_login))
}

async fn render_login(Extension(state): Extension<Arc<AppState>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("username", "user");
    context.insert("password", "pass");
    let state=get_app_state(&state);
    let rendered = state.tera.render("auth/login.html", &context).unwrap();
    Html(rendered)
}