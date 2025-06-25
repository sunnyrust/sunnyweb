/// index controller
use axum::{
    routing::{get, post},
    Router,
    Extension,
    
    response::{Html,Redirect, IntoResponse},
    extract::{Form,Query, State},
    http::{StatusCode, header},
};
use captcha_rs::CaptchaBuilder;
use crate::{AppState,controller::get_app_state,utils::*,BaseController,get_translation};
use std::sync::Arc;
use tera::{ Context};
use tower_sessions::{Session};
use serde::Deserialize;

pub fn router() -> Router {
    Router::new()
        .route("/", get(render))
}

async fn render(
    Extension(state): Extension<Arc<AppState>>,
    session: Session
    ) -> Html<String> {
    tracing::info!("Index……😀");
    let mut ctx = Context::new();
    // ctx.insert("username", "user");
    // ctx.insert("password", "pass");
    // let captcha_image = generate_captcha_image(session).await;
    // ctx.insert("captcha_image", &captcha_image);
    if let Some(trans) = get_translation("zh-CN") {
        ctx.insert("trans", trans);
    }

    let state = get_app_state(&state);
    let rendered = state.tera.render("index/index.html", &ctx).unwrap();
    Html(rendered)
}