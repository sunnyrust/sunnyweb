use crate::model::users::*;
use crate::{AppError, AppState, Result,model::users};

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
use sunny_derive_trait::PgCurdStruct;
use crate::{controller::get_app_state,utils::*,BaseController,get_translation};
use std::sync::Arc;
use tera::{ Context};
use tower_sessions::{Session};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/add", get(add).post(do_insert))
        // .route("/add", post(insert_one))
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
    let user=crate::model::users::Model::default();
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
    let editor=crate::model::users::Model::default();
    ctx.insert("editor", &editor);
    let state = get_app_state(&state);
    let rendered = state.tera.render("user/form.html", &ctx).unwrap();
    Html(rendered)
}
#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct UserAddForm {
    pub id: Option<i32>,
    pub name: String,
    pub password: String,
    pub email: String,
    pub username: String,
    pub description: String,
    pub is_active: Option<String>,
}
/// insert a new user
async fn do_insert(
    Extension(state): Extension<Arc<AppState>>,
    Extension(base_controller): Extension<BaseController>,
    Form(user): Form<UserAddForm>,
) ->  core::result::Result<Redirect, crate::utils::types::HtmlResponse> {
    tracing::info!("User Add……😀");
    let mut ctx = Context::new();
    let mut jump_message = message::JumpMessage {
        title: "User Add".to_string(),
        staus: true,
        wait: 3,
        message: "Success".to_string(),
        url: "/login/test".to_string(),
        platform_token: "".to_string(),
    };
    let trans = get_translation("zh-CN").unwrap();
    ctx.insert("trans", trans);
    let is_active = user.is_active.is_some();
    tracing::info!("is_active:{}", is_active);
    ctx.insert("getversion", base_controller.app_version.as_str());
    let state = get_app_state(&state);
    if user.name.is_empty() {
        jump_message.staus = false;
        jump_message.message = trans["editor"]["form"]["username_empty_error"].to_string();
        jump_message.url = "/user/add".to_string();
        ctx.insert("jump_message", &jump_message);
        Err(super::webhotel_render(state.tera.clone(), ctx, "common/message.html"))
    }else{
        let user_model  = users::Model{
            id: user.id.unwrap_or(0),
            name: user.name,
            password_hash: user.password, // In a real application, you should hash the password
            email: user.email,
            username: user.username.clone(),
            description: user.description,
            is_active: is_active,
        };
        let res = users::insert_one(&state, &user_model.insert(),&user_model.get_table_name().to_string(),&user.username).await;
        match res {
            Ok(_) => {
                jump_message.staus = true;
                jump_message.message = String::from("Ok");
                jump_message.url = "/user/list".to_string();
            }
            Err(err) => {
                jump_message.staus = false;
                let _msg = match err.error {
                    crate::err::AppErrorItem::Cause(err) => err.to_string(),
                    crate::err::AppErrorItem::Message(msg) => msg.unwrap_or("发生错误".to_string()),
                };
                tracing::error!("Error inserting user: {}", _msg);
                jump_message.message = _msg;
                jump_message.url = "/user/add".to_string();
                
            }
        }
        if jump_message.staus {
            ctx.insert("jump_message", &jump_message);
            return Ok(Redirect::to("/user/list"));
        } else {
            ctx.insert("jump_message", &jump_message);
            return Err(super::webhotel_render(state.tera.clone(), ctx, "common/message.html"));
        }
        // Ok(Redirect::to("/user/list"))
    }

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
    let mut tpl = crate::model::users::Model::default();
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