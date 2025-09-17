pub mod login;
pub mod index;
pub mod user;
pub mod navigation;
use crate::{AppState,AppError,Result,config::WebHotelInfo};

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
fn get_web_info<'a>(webinfo: &'a WebHotelInfo) -> WebHotelInfo{
   webinfo.to_owned()
}

#[allow(dead_code)]
async fn get_language(info: &WebHotelInfo, session: tower_sessions::Session) -> serde_json::Value {
   let info = get_web_info(&info);
   let lang = match session.get::<String>("language").await {
        Ok(Some(language)) => {
            tracing::info!("Language retrieved from session: {}", language);
            language
        }
        Ok(None) => {
            tracing::info!("No language found in session, using default.");
            info.default.clone() // Use the default language from info
        }
        Err(e) => {
            tracing::error!("Error retrieving language from session: {:?}", e);
            info.default.clone() // Use the default language on error as well
        }
    };
    crate::get_translation(&lang).unwrap().clone()
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