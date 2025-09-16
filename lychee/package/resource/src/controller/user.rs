use crate::model::users::*;
use crate::{
    controller::{get_app_state, get_web_info,get_language},
    BaseController,get_translation,
    AppError, AppState, Result,
    model::users,
    utils::{password::PasswordHasher,*},
    config::WebHotelInfo
};

use axum::extract::Path;
/// index controller
use axum::{
    routing::{get, post},
    Router,
    Extension,
    
    response::{Html,Redirect, IntoResponse},
    extract::{Form,Query, State},
    http::{StatusCode, header},
};
use sunny_derive_trait::PgCurdStruct;
use std::f64::consts::E;
use std::sync::Arc;
use tera::{ Context};
use tower_sessions::{Session};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/add", get(add).post(do_insert))
        // .route("/add", post(insert_one))
        .route("/list", get(list))
        .route("/edit/id/{id}", get(edit).post(do_update))
        .route("/del/id/{id}", get(do_delete))
}

async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Extension(base_controller): Extension<BaseController>,
    Extension(info): Extension<Arc<WebHotelInfo>>,
    session: Session
    ) -> Html<String> {
    tracing::info!("User……😀");
    let mut ctx = Context::new();
    let info=get_web_info(&info);
    // let mut lang=session.get::<String>("language").await.unwrap().unwrap();
    // if lang.is_empty() {
    //     lang = info.default;
    // }
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
    if let Some(trans) = get_translation(&lang) {
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
    Extension(info): Extension<Arc<WebHotelInfo>>,
    Path(id): Path<i32>,
    session: Session
    ) -> Html<String> {
    tracing::info!("User Editor Controller ......……😀{}", id);
    let mut ctx = Context::new();
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
    if let Some(trans) = get_translation(&lang) {
        ctx.insert("trans", trans);
    }
    ctx.insert("action_name", "Edit");
    ctx.insert("getversion", base_controller.app_version.as_str());
    let user=crate::model::users::Model::default();
    let editor=match crate::model::users::get_one(&state,&user.get_one_by_id(id)).await{
        Ok(editor) => editor,
        Err(e) => {
            tracing::error!("Error retrieving editor: {:?}", e);
            let jump_message = message::JumpMessage {
                title: "User Edit".to_string(),
                staus: false,
                wait: 3,
                message: e.to_string(),
                url: "/user/list".to_string(),
                platform_token: "".to_string(),
            };
            ctx.insert("jump_message", &jump_message);
            return Html(state.tera.render("common/message.html", &ctx).unwrap());
        }
    };
    // tracing::error!("Get editor success{:?}", editor);
    ctx.insert("user", &editor);
    let state = get_app_state(&state);
    let rendered = state.tera.render("user/form.html", &ctx).unwrap();
    Html(rendered)
}
#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct UserAddForm {
    pub id: Option<i32>,
    pub name: String,
    pub password: String,
    pub re_password: String,
    pub email: String,
    pub username: String,
    pub description: String,
    pub is_active: Option<String>,
}
/// insert a new user
async fn do_insert(
    Extension(state): Extension<Arc<AppState>>,
    Extension(base_controller): Extension<BaseController>,
    Extension(info): Extension<Arc<WebHotelInfo>>,
    session: Session,
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
    let trans = get_translation(&lang).unwrap();
    ctx.insert("trans", trans);
    let is_active = user.is_active.is_some();
    tracing::info!("is_active:{}", is_active);
    ctx.insert("getversion", base_controller.app_version.as_str());
    let state = get_app_state(&state);
    if user.password!=user.re_password {
        jump_message.staus = false;
        jump_message.message = trans["editor"]["passwordMatch"].to_string();
        jump_message.url = "/user/add".to_string();
        ctx.insert("jump_message", &jump_message);
        return Err(super::webhotel_render(state.tera.clone(), ctx, "common/message.html"));
    }
    if user.username.is_empty() {
        jump_message.staus = false;
        jump_message.message = trans["editor"]["form"]["username_empty_error"].to_string();
        jump_message.url = "/user/add".to_string();
        ctx.insert("jump_message", &jump_message);
        Err(super::webhotel_render(state.tera.clone(), ctx, "common/message.html"))
    }else{
        let password_hash = PasswordHasher::new().hash(user.password.as_str());
        let user_model  = users::Model{
            id: user.id.unwrap_or(0),
            name: user.name,
            password_hash: password_hash,
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
async fn do_update(
    Extension(state): Extension<Arc<AppState>>,
    Extension(base_controller): Extension<BaseController>,
    Extension(info): Extension<Arc<WebHotelInfo>>,
    session: Session,
    Form(user): Form<UserAddForm>,
) ->  core::result::Result<Redirect, crate::utils::types::HtmlResponse> {
    tracing::info!("User Update……😀");
    let mut ctx = Context::new();
    let mut jump_message = message::JumpMessage {
        title: "User Update".to_string(),
        staus: true,
        wait: 3,
        message: "Success".to_string(),
        url: "/login/test".to_string(),
        platform_token: "".to_string(),
    };
    // let info = get_web_info(&info);

    // let lang = match session.get::<String>("language").await {
    //     Ok(Some(language)) => {
    //         tracing::info!("Language retrieved from session: {}", language);
    //         language
    //     }
    //     Ok(None) => {
    //         tracing::info!("No language found in session, using default.");
    //         info.default.clone() // Use the default language from info
    //     }
    //     Err(e) => {
    //         tracing::error!("Error retrieving language from session: {:?}", e);
    //         info.default.clone() // Use the default language on error as well
    //     }
    // };
    // let trans = get_translation(&lang).unwrap();
    let trans = get_language(&info,session).await;
    ctx.insert("trans", &trans);
    let is_active = user.is_active.is_some();
    ctx.insert("getversion", base_controller.app_version.as_str());
    let state = get_app_state(&state);
    if user.password!=user.re_password {
        jump_message.staus = false;
        jump_message.message = trans["editor"]["passwordMatch"].to_string();
        jump_message.url = format!("/user/edit/id/{}", user.id.unwrap());
        ctx.insert("jump_message", &jump_message);
        return Err(super::webhotel_render(state.tera.clone(), ctx, "common/message.html"));
    }
    let mut user_model  = users::Model{
        id: user.id.unwrap_or(0),
        name: user.name,
        password_hash: "".to_string(),
        email: user.email,
        username: user.username.clone(),
        description: user.description,
        is_active: is_active,
    };
    let mut str_sql = "".to_string();
    if !user.password.is_empty() {
        let password_hash = PasswordHasher::new().hash(user.password.as_str());
        user_model.password_hash = password_hash;
        str_sql = user_model.update(user.id.unwrap());
    }else{
        use regex::Regex;
        let re = Regex::new(r",\s*password_hash\s*=\s*'[^']*'").unwrap();
        let sql = user_model.update(user.id.unwrap());
        str_sql = re.replace_all(&sql, "").to_string();
    }
    
    let res = users::update(&state, &str_sql).await;
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
            jump_message.url = format!("/user/edit/id/{}", user.id.unwrap());

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

/// Delete a user
async fn do_delete(
    Extension(state): Extension<Arc<AppState>>,
    Extension(base_controller): Extension<BaseController>,
    Extension(info): Extension<Arc<WebHotelInfo>>,
    session: Session,
    Path(id): Path<i32>,
) ->  core::result::Result<Redirect, crate::utils::types::HtmlResponse> {
    tracing::info!("User Delete……😀");
    let mut ctx = Context::new();
    let mut jump_message = message::JumpMessage {
        title: "User Delete".to_string(),
        staus: true,
        wait: 3,
        message: "Success".to_string(),
        url: "/user/list".to_string(),
        platform_token: "".to_string(),
    };
    let trans = get_language(&info,session).await;
    ctx.insert("trans", &trans);
    ctx.insert("jump_message", &jump_message);
    let mut user_model  = users::Model::default();
    user_model.id = id;
    users::delete(&state, &user_model.delete(id)).await.unwrap();
    Ok(Redirect::to("/user/list"))
}
/// List users
async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Extension(base_controller): Extension<BaseController>,
    Extension(info): Extension<Arc<WebHotelInfo>>,
    session: Session,
) -> Html<String> {
    tracing::info!("User……😀");
    let mut ctx = Context::new();
    let info= get_web_info(&info);
    // let mut lang=session.get::<String>("language").await.unwrap().unwrap();
    // if lang.is_empty() {
    //     lang = info.default;
    // }
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
    if let Some(trans) = get_translation(&lang ) {
        ctx.insert("trans", trans);
    }
    ctx.insert("action_name", "List");
    ctx.insert("getversion", base_controller.app_version.as_str());
    
    let model  = users::Model::default();
    let user_list = crate::model::users::get_all(&state, &model.select()).await.unwrap_or_else(|_| vec![]);
    ctx.insert("user_list", &user_list);
    ctx.insert("user", &model);

    let state = get_app_state(&state);
    let rendered = state.tera.render("user/list.html", &ctx).unwrap();
    Html(rendered)
}