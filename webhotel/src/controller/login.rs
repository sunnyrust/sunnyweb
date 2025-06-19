//login controller
use axum::{
    routing::get, 
    Router, 
    Extension,
    response::{Html, IntoResponse},
    extract::{Query, State},
    http::{StatusCode, header},
};
use tera::{ Context};
use crate::{AppState,controller::get_app_state,BaseController};
use std::sync::{Arc, Mutex};
use captcha_rs::CaptchaBuilder;
use tower_sessions::{Session};

pub fn login_router() -> Router {
    Router::new()
        .route("/login", get(render_login))
        .route("/captcha", get(generate_captcha_image))
        .route("/test", get(test))
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
    
    let mut context = Context::new();
    context.insert("module", &module);
    context.insert("function", &function);
    let captcha_text = session.get::<String>("captcha_text").await.or_else(|_| Err("No captcha text found".to_string()));

    context.insert("captcha_text", &captcha_text);
    
    let rendered = state.tera.render("auth/test.html", &context).unwrap();
    Html(rendered)
}


async fn render_login(Extension(state): Extension<Arc<AppState>>,session: Session) -> Html<String> {
     tracing::info!("Login……😀");
    let mut context = Context::new();
    context.insert("username", "user");
    context.insert("password", "pass");
    let captcha_image = generate_captcha_image(session).await;
    context.insert("captcha_image", &captcha_image);
   
    let state=get_app_state(&state);
    let rendered = state.tera.render("auth/login.html", &context).unwrap();
    Html(rendered)
}
// generate captcha image
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