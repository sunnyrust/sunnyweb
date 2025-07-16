//login controller
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
        .route("/login", get(render_login))
        .route("/login", post(login))
        .route("/captcha", get(generate_captcha_image))
        .route("/test", get(test))
}



#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub captcha: String,
}
pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Extension(base_controller): Extension<BaseController>,
    session: Session,
    Form(form): Form<LoginForm>,
) ->  Result<Redirect, Html<String>>  {
    tracing::info!("😀Login attempt: username:{:?},password:{:?},captcha:{:?}",form.username, form.password, form.captcha);
    let captcha_text = session
    .get::<String>("captcha_text")
    .await
    .unwrap_or_else(|_| Some("default_captcha".to_string()))
    .unwrap();
    let message = format!("😀Login attempt: username:{:?},password:{:?},captcha:{:?}\r\n😀Captcha text from session: {}", form.username, form.password, form.captcha, captcha_text);
    
    tracing::info!(message);
    let mut jump_message = message::JumpMessage {
        title: "Login".to_string(),
        staus: true,
        wait: 3,
        message: "登录成功".to_string(),
        url: "/login/test".to_string(),
        platform_token: "".to_string(),
    };
    // 模拟验证逻辑
    let mut ctx = Context::new();
    ctx.insert("getversion", base_controller.app_version.as_str());
    if form.captcha == captcha_text {
        ctx.insert("message", &jump_message);
        Ok(Redirect::to("/index"))
    } else {
        
        ctx.insert("username", form.username.as_str());
        ctx.insert("password", "");
        let captcha_image = generate_captcha_image(session).await;
        ctx.insert("captcha_image", &captcha_image);
        if let Some(trans) = get_translation("en-US") {
            ctx.insert("trans", trans);
            jump_message.staus = false;
            jump_message.message = trans["login"]["form"]["captcha_error"].to_string();
            jump_message.url = "/login/login".to_string();
        }
        ctx.insert("jump_message", &jump_message);
        //Err(Html(state.tera.render("common/message.html", &ctx).unwrap()))
        Err(super::webhotel_render(state.tera.clone(), ctx, "common/message.html"))
    }
}

/// generate captcha image
async fn generate_captcha_image(
    session: Session,
) -> String {
    let captcha = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(50)
        .complexity(3)
        .build();

    // save session
    session.insert("captcha_text", captcha.text.clone()).await.unwrap();

    // Convert the captcha image to a base64 string for embedding in HTML   
    let base64_image = captcha.to_base64();
    let img_src = format!("{}", base64_image);
    img_src
}
async fn render_login(
    Extension(state): Extension<Arc<AppState>>,
    session: Session
    ) -> Html<String> {
    tracing::info!("Login……😀");
    let mut ctx = Context::new();
    ctx.insert("username", "user");
    ctx.insert("password", "pass");
    let captcha_image = generate_captcha_image(session).await;
    ctx.insert("captcha_image", &captcha_image);
    if let Some(trans) = get_translation("zh-CN") {
        ctx.insert("trans", trans);
    }

    let state = get_app_state(&state);
    let rendered = state.tera.render("auth/login.html", &ctx).unwrap();
    Html(rendered)
}
async fn test(
    uri: axum::extract::OriginalUri,
    Extension(base_controller): Extension<BaseController>,
    Extension(state): Extension<Arc<AppState>>,
    session: Session
) -> Html<String> {
    tracing::info!("Testing……😀");
    tracing::info!("😀😀User path: {}", base_controller.uri.path());
    // let segments = state.path_segments.lock().unwrap().clone();
    let full_path = uri.0.path().to_string();
    let path_segments: Vec<String> = full_path
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let module = path_segments.get(0).cloned().unwrap_or("default".into());
    let function = path_segments.get(1).cloned().unwrap_or("function".into());
    // let segments = &state.path_segments;
    //  let segments: MutexGuard<'_, Vec<String>> = state.path_segments.lock().await;
    // tracing::info!("{:?}", segments);
    
    let mut ctx = Context::new();
    ctx.insert("module", &module);
    ctx.insert("function", &function);
    let captcha_text = session.get::<String>("captcha_text").await.or_else(|_| Err("No captcha text found".to_string()));

    ctx.insert("captcha_text", &captcha_text);
    
    let rendered = state.tera.render("auth/test.html", &ctx).unwrap();
    Html(rendered)
}