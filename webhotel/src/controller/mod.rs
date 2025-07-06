pub mod login;
pub mod index;
pub mod user;
use crate::{AppState,AppError,Result};

use askama::Template;
use axum::{
   response::Html,
};
#[allow(dead_code)]
/// get app state
pub fn get_app_state<'a>(state: &'a AppState) -> &'a AppState {
    state
}
#[allow(dead_code)]
/// 渲染模板
fn render<T: Template>(tpl: T, handler_name: &str) -> Result<super::utils::types::HtmlResponse> {
   let out = tpl
       .render()
       .map_err(AppError::from)
       .map_err(log_error(handler_name))?;
   Ok(Html(out))
}
#[allow(dead_code)]
/// 记录错误
fn log_error(handler_name: &str) -> Box<dyn Fn(AppError) -> AppError> {
   let handler_name = handler_name.to_string();
   Box::new(move |err| {
       tracing::error!("{}: {:?}", handler_name, err);
       err
   })
}

fn webhotel_render(tera:tera::Tera,ctx :tera::Context,template:&str)->super::utils::types::HtmlResponse{
    Html(tera.render(template, &ctx).unwrap())
}