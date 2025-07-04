use crate::model::user::*;
use crate::{AppError, AppState, Result};

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
use crate::{controller::get_app_state,utils::*,BaseController,get_translation};
use std::sync::Arc;
use tera::{ Context};
use tower_sessions::{Session};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/add", get(add))
        .route("/add", post(insert_one))
        .route("/list", get(list))
        .route("/edit", get(edit))
}

async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Extension(base_controller): Extension<BaseController>,
    session: Session
    ) -> Html<String> {
    tracing::info!("User……😀");
    let mut ctx = Context::new();
    // ctx.insert("username", "user");
    // ctx.insert("password", "pass");
    // let captcha_image = generate_captcha_image(session).await;
    // ctx.insert("captcha_image", &captcha_image);
    if let Some(trans) = get_translation("zh-CN") {
        ctx.insert("trans", trans);
    }
    ctx.insert("action_name", "Add");
    ctx.insert("getversion", base_controller.app_version.as_str());
    let user=crate::model::user::Model::default();
    ctx.insert("user", &user);
    let state = get_app_state(&state);
    let rendered = state.tera.render("user/form.html", &ctx).unwrap();
    Html(rendered)
}

async fn edit(
    Extension(state): Extension<Arc<AppState>>,
    Extension(base_controller): Extension<BaseController>,
    session: Session
    ) -> Html<String> {
    tracing::info!("User……😀");
    let mut ctx = Context::new();
    // ctx.insert("username", "user");
    // ctx.insert("password", "pass");
    // let captcha_image = generate_captcha_image(session).await;
    // ctx.insert("captcha_image", &captcha_image);
    if let Some(trans) = get_translation("zh-CN") {
        ctx.insert("trans", trans);
    }
    ctx.insert("action_name", "Edit");
    ctx.insert("getversion", base_controller.app_version.as_str());
    let editor=crate::model::user::Model::default();
    ctx.insert("editor", &editor);
    let state = get_app_state(&state);
    let rendered = state.tera.render("user/form.html", &ctx).unwrap();
    Html(rendered)
}
#[derive(Serialize,Deserialize)]
pub struct UserAddForm {
    pub id: Option<i32>,
    pub name: String,
    pub password: String,
    pub email: String,
    pub username: String,
    pub description: String,
    pub is_active: bool,
}
/// insert a new user
async fn insert_one(
    Extension(state): Extension<Arc<AppState>>,
    Extension(base_controller): Extension<BaseController>,
    session: Session,
    Form(user): Form<UserAddForm>,
) -> Result<Html<String>> {
    tracing::info!("User Add……😀");
    let mut ctx = Context::new();
    if let Some(trans) = get_translation("zh-CN") {
        ctx.insert("trans", trans);
    }

    ctx.insert("getversion", base_controller.app_version.as_str());
    ctx.insert("user", &user);
    let state = get_app_state(&state);
    let rendered = state.tera.render("user/form.html", &ctx).unwrap();
    Ok(Html("rendered".to_string()))
    // Here you would typically insert the user into the database
}

/// List users
async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Extension(base_controller): Extension<BaseController>,
    session: Session,
) -> Html<String> {
    tracing::info!("User……😀");
    let mut ctx = Context::new();
    if let Some(trans) = get_translation("zh-CN") {
        ctx.insert("trans", trans);
    }
    ctx.insert("action_name", "List");
    ctx.insert("getversion", base_controller.app_version.as_str());
    let mut tpl = crate::model::user::Model::default();
    tpl.id = 1; // Example ID, replace with actual logic to fetch user data
    tpl.name = "Example User".to_string(); // Example name, replace with actual logic
    tpl.email = "example@example.com".to_string(); // Example email, replace with actual logic
    tpl.username = "exampleuser".to_string(); // Example username, replace with actual logic
    tpl.description = "This is an example user description.".to_string(); // Example description
    tpl.is_active = true;
    let mut user_list = Vec::new();
    user_list.push(tpl.clone());
    ctx.insert("user_list", &user_list);
    ctx.insert("user", &tpl);

    let state = get_app_state(&state);
    let rendered = state.tera.render("user/list.html", &ctx).unwrap();
    Html(rendered)
}