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
use crate::AppState;
use std::sync::Arc;
use crate::controller::get_app_state;
use captcha_rs::CaptchaBuilder;
use tower_sessions::{Session};
pub fn login_router() -> Router {
    Router::new()
        .route("/login", get(render_login))
        .route("/captcha", get(generate_captcha_image))
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