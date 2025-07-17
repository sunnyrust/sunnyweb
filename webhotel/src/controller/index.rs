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
use serde::{Deserialize,Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/", get(render))
}
#[derive(Deserialize,Serialize,Debug, Clone)]
pub struct  Usr{
    pub username: String,
    pub user_id: i32,
    pub some_url: String,
    pub some_target: String,
    pub has_power: bool,
}
impl Default for Usr {
    fn default() -> Self {
        Usr {
            username: "Guest".to_string(),
            user_id: -1,
            some_url: "https://www.baidu.com".to_string(),
            some_target: "百度".to_string(),
            has_power: true,
        }
    }
    
}
async fn render(
    Extension(state): Extension<Arc<AppState>>,
    session: Session
    ) -> Html<String> {
    tracing::info!("Index……😀");
    let mut ctx = Context::new();
    if let Some(trans) = get_translation("en-US") {
        ctx.insert("trans", trans);
    }
    let mut usr = Usr::default();
    let u_name = session.get::<String>("username").await.unwrap_or(Some("Guest".to_string()));
    usr.username = u_name.unwrap();
    let u_id= session.get::<i32>("id").await.unwrap_or(Some(-1));
    usr.user_id = u_id.unwrap();
    ctx.insert("usr", &usr);
    
    let state = get_app_state(&state);
    let rendered = state.tera.render("index/index.html", &ctx).unwrap();
    Html(rendered)
}