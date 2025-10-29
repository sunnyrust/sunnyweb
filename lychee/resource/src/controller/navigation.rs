use super::render;
use crate::{ model::navigation::*, model::*, utils::stack::*, 
    utils::types::*, utils::*,
    AppState,
    controller::{get_app_state, get_web_info,get_language}
};
use tera::{ Context};
use askama::Template;
use axum::{
    extract::Path,
    response::{Html,Redirect, IntoResponse},
    http::{header::HeaderName, HeaderMap, HeaderValue, StatusCode},
    routing::{get, post},
    Extension, Form, Json, Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
pub(crate) fn router() -> Router {
    Router::new()
        .route("/list", get(list))
        .route("/tree", get(get_tree))
        .route("/find/{id}", get(get_one))
        .route("/edit/{id}", get(edit))
        .route("/doedit", post(do_edit))
        .route("/add", get(add).post(do_add))
        .route("/del/{id}", get(do_del))
        .layer(TraceLayer::new_for_http())
}

async fn get_one(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> types::HandlerJsonResult {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );

    let tag = navigation::get_one_by_id(&state, id).await.unwrap();
    let result = Json(serde_json::json!({ "result": tag }));
    let code = StatusCode::OK;
    (code, headers, result)
}

/// 取得tag树
async fn get_tree(Extension(state): Extension<Arc<AppState>>) -> types::HandlerJsonResult {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );

    let tag = navigation::get_navigation_tree(&state).await.unwrap();
    let result = Json(serde_json::json!({ "result": tag }));
    let code = StatusCode::OK;
    (code, headers, result)
}

// #[derive(Template)]
// #[template(path = "navigation/delete.html")]
// pub struct TagDeleteTemplate {}
// /// 显示del页面
// async fn del_form() -> crate::Result<HtmlResponse> {
//     let handler_name = "DeleteTagById";

//     let tpl = TagDeleteTemplate {};
//     render(tpl, handler_name)
// }

// #[derive(Debug, Clone, PartialEq, Deserialize, sqlx::FromRow)]
// pub struct DelForm {
//     pub id: i32,
// }
pub async fn do_del(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> core::result::Result<Redirect, crate::utils::types::HtmlResponse> {
    let mut ctx = Context::new();
    let mut jump_message = message::JumpMessage {
        title: "Navigation Add".to_string(),
        status: true,
        wait: 3,
        message: "Success".to_string(),
        url: "/navigation/list".to_string(),
        platform_token: "".to_string(),
    };
    #[allow(unused_assignments)]
    let message = String::from("OK");
    let msg = navigation::delete(&state, id).await;
    match msg {
        Ok(msg) => {
            // message = msg;
            // jump_message.status = true;
            // jump_message.message = message;
            // ctx.insert("jump_message", &jump_message);
            return Ok(Redirect::to("/navigation/list"));
        },
        Err(e) => {
            let msg = match e.error {
                crate::err::AppErrorItem::Cause(err) => err.to_string(),
                crate::err::AppErrorItem::Message(msg) => msg.unwrap_or("发生错误".to_string()),
            };
            // message = msg;
            jump_message.status = false;
            jump_message.message = msg;
            ctx.insert("jump_message", &jump_message);
            return Err(super::webhotel_render(state.tera.clone(), ctx, "common/message.html"));
        }
    }
    // let handler_name = "Message";
    // let mut tpl = crate::utils::message::JumpMessage::default();
    // tpl.title = "Delete".to_string();
    // tpl.message = message;
    // tpl.status = true;
    // tpl.url = "/navigation/list".to_string();
    // super::render(tpl, handler_name)
}

#[derive(Template)]
#[template(path = "navigation/list.html", ext = "html", escape = "none")]
pub struct ListForm<'a> {
    pub ul: &'a str,
}

pub async fn list(Extension(state): Extension<Arc<AppState>>) -> Result<HtmlResponse> {
    let tags = navigation::get_navigation_tree(&state).await.unwrap();
    let handler_name = "list";
    let mut str_html = String::from("");
    let mut stack = Stack::new();
    for tag in tags {
        str_html = set_html(&mut stack, &tag, str_html, true);
    }
    if !stack.is_empty() {
        loop {
            let l_stack = stack.pop().unwrap();
            str_html += r#"</li></ul>"#;
            if l_stack.level == 0 {
                break;
            }
        }
    }
    // tracing::debug!("{str_html}");
    let tpl = ListForm { ul: &str_html };
    render(tpl, handler_name)
}
#[derive(Template)]
#[template(path = "navigation/edit.html", ext = "html", escape = "none")]
pub struct EditForm<'a, 'b> {
    pub ul: &'a str,
    pub id: i32,
    pub pid: i32,
    pub name: &'b str,
    pub is_display: bool,
}
pub async fn edit(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<HtmlResponse> {
    let tags = navigation::get_navigation_tree(&state).await.unwrap();
    let mut node_map: HashMap<i32, NavigationModel> = HashMap::new();
    for node in tags.clone() {
        node_map.insert(node.id, node);
    }
    let tag_root = node_map.get(&id);
    let tag_root = match tag_root {
        Some(val) => val.to_owned(),
        None => NavigationModel {
            id: 0,
            name: "Root".to_string(),
            pid: 0,
            level: -9,
            is_parent: true,
            is_display: true,
        },
    };
    if tag_root.level == -9 {
        let mut tpl = crate::utils::message::JumpMessage::default();
        tpl.status = false;
        tpl.title = "Error".to_string();
        tpl.message = String::from("Data not found");
        tpl.url = "/navigation/list".to_string();
        // return super::render(tpl, "Message");
        Ok(super::webhotel_render(state.tera.clone(), Context::new(), "common/message.html"))
    } else {
        let mut ids = Vec::new();
        tag_root.traverse(&node_map, &mut ids);
        let handler_name = "edit";
        let mut str_html = String::from("");
        let mut stack = Stack::new();
        for tag in tags {
            if !ids.contains(&tag.id) || id == 0 {
                str_html = set_html(&mut stack, &tag, str_html, false);
            }
        }
        if !stack.is_empty() {
            loop {
                let l_stack = stack.pop().unwrap();
                str_html += r#"</li></ul>"#;
                if l_stack.level == 0 {
                    break;
                }
            }
        }
        let m = navigation::get_one_by_id(&state, id).await.unwrap_or(Model {
            id: 0,
            name: "".to_string(),
            pid: 0,
            is_display: false,
        });
        let tpl = EditForm {
            ul: &str_html,
            id: id,
            pid: m.pid,
            name: &m.name,
            is_display: m.is_display,
        };
        super::render(tpl, handler_name)
    }
}

/// 判断是不是要压栈
fn judge_push(stack: &Stack<NavigationModel>, tml: &NavigationModel) -> bool {
    #[allow(unused_assignments)]
    let mut result = false;
    if tml.level == -1 {
        return true;
    }
    let old = stack.get_pop();
    let o = match old {
        Some(val) => val.to_owned(),
        None => NavigationModel {
            id: 0,
            name: "Root".to_string(),
            pid: 0,
            level: -9,
            is_parent: true,
            is_display: true,
        },
    };
    if o.level == -9 {
        result = true;
    } else {
        if tml.pid == o.id {
            result = true;
        } else {
            result = false
        }
    }
    result
}

/// 拼接Tree的html字符串
fn set_html(stack: &mut Stack<NavigationModel>, tml: &NavigationModel, html: String, have_link: bool) -> String {
    let mut str_html = html;
    #[allow(unused_assignments)]
    let mut s = String::new();
    if judge_push(&stack, &tml) {
        if have_link {
            let mut display="👁︎".to_string();
            if !tml.is_display {
                display="🚫".to_string();
            }
            s = format!(
                "<ul><li id='tree_{}' pid='{}'><a href='./edit/{}'>{}{}</a>",
                tml.id, tml.pid, tml.id, tml.name,display
            );
        } else {
            s = format!(
                "<ul><li id='tree_{}' pid='{}'>{}",
                tml.id, tml.pid, tml.name
            );
        }
        str_html += &s;
        stack.push(tml.to_owned());
    } else {
        if !stack.is_empty() {
            loop {
                let l_stack = stack.pop().unwrap();
                str_html += r#"</li>"#;
                if l_stack.pid == tml.pid {
                    break;
                }
                str_html += r#"</ul>"#;
            }
            if have_link {
                let mut display="👁︎".to_string();
                if !tml.is_display {
                    display="🚫".to_string();
                }
                s = format!(
                    "<li id='tree_{}' pid='{}'><a href='./edit/{}'>{}{}</a>",
                    tml.id, tml.pid, tml.id, tml.name,display
                );
            } else {
                s = format!("<li id='tree_{}' pid='{}'>{}", tml.id, tml.pid, tml.name);
            }
            str_html += &s;
            stack.push(tml.to_owned());
        }
    }

    str_html
}

#[derive(Deserialize)]
pub struct EditStruct {
    pub id: i32,
    pub pid: i32,
    pub name: String,
    pub is_display: Option<String>,
}

pub async fn do_edit(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<EditStruct>,
) -> core::result::Result<Redirect, crate::utils::types::HtmlResponse> {
    let mut ctx = Context::new();
    let mut jump_message = message::JumpMessage {
        title: "Navigation Add".to_string(),
        status: true,
        wait: 3,
        message: "Success".to_string(),
        url: "/navigation/list".to_string(),
        platform_token: "".to_string(),
    };
    let result = navigation::update(
        &state,
        Model {
            id: frm.id,
            name: frm.name.clone(),
            pid: frm.pid,
            is_display: frm.is_display.is_some(),
        },
    )
    .await;
    match result {
        Ok(_) => {
            jump_message.status = true;
            jump_message.message = "OK".to_string();
        }
        Err(e) => {
            let msg = match e.error {
                crate::err::AppErrorItem::Cause(err) => err.to_string(),
                crate::err::AppErrorItem::Message(msg) => msg.unwrap_or("An error occurred.".to_string()),
            };
            jump_message.status = false;
            jump_message.message = msg;
        }        
    }
    if jump_message.status {
        ctx.insert("jump_message", &jump_message);
        return Ok(Redirect::to("/navigation/list"));
    } else {
        ctx.insert("jump_message", &jump_message);
        return Err(super::webhotel_render(state.tera.clone(), ctx, "common/message.html"));
    }

    // let handler_name = "Message";
    // #[allow(unused_assignments)]
    // let mut message = String::from("OK");
    // let mut tpl = crate::view::MsgTemplate::default();
    // match result {
    //     Ok(msg) => {
    //         message = msg;
    //         tpl.is_success = true;
    //     }
    //     Err(e) => {
    //         let _msg = match e.error {
    //             crate::err::AppErrorItem::Cause(err) => err.to_string(),
    //             crate::err::AppErrorItem::Message(msg) => msg.unwrap_or("发生错误".to_string()),
    //         };
    //         message = _msg;
    //         tpl.is_success = false;
    //     }
    // }
    // tpl.msg = message;
    // tpl.target_url = Some("/navigation/list".to_string());

    // render(tpl, handler_name)
}

// method: Method,
// if method.as_str()=="post"{

//     return "Method Not Allowed".to_string();
// }
#[derive(Template)]
#[template(path = "navigation/add.html", ext = "html", escape = "none")]
pub struct AddFormTemplate<'a, 'b> {
    pub ul: &'a str,
    pub id: i32,
    pub pid: i32,
    pub name: &'b str,
    pub is_display: bool,
}
pub async fn add(
    Extension(state): Extension<Arc<AppState>>
)-> Result<HtmlResponse> {
    let tags = navigation::get_navigation_tree(&state).await.unwrap();
    let handler_name = "Add";
    let mut str_html = String::from("");
    let mut stack = Stack::new();
    for tag in tags {
        str_html = set_html(&mut stack, &tag, str_html, false);
    }
    if !stack.is_empty() {
        loop {
            let l_stack = stack.pop().unwrap();
            str_html += r#"</li></ul>"#;
            if l_stack.level == 0 {
                break;
            }
        }
    }
    let tpl = AddFormTemplate {
        ul: &str_html,
        id: -1,
        pid: 0,
        name: "",
        is_display: true,
    };
    render(tpl, handler_name)
}

#[derive(Deserialize)]
pub struct AddStruct {
    pub id: i32,
    pub pid: i32,
    pub name: String,
    pub is_display: Option<String>,
}
pub async fn do_add(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<AddStruct>,
) -> core::result::Result<Redirect, crate::utils::types::HtmlResponse> {
    let mut ctx = Context::new();
    let mut jump_message = message::JumpMessage {
        title: "Navigation Add".to_string(),
        status: true,
        wait: 3,
        message: "Success".to_string(),
        url: "/navigation/list".to_string(),
        platform_token: "".to_string(),
    };

    let result = navigation::insert_one(
        &state,
        Model {
            id: frm.id,
            name: frm.name.clone(),
            pid: frm.pid,
            is_display: frm.is_display.is_some(),
        },
    )
    .await;
    match result {
        Ok(_) => {
            jump_message.status = true;
            jump_message.message = "OK".to_string();
        }
        Err(e) => {
            let msg = match e.error {
                crate::err::AppErrorItem::Cause(err) => err.to_string(),
                crate::err::AppErrorItem::Message(msg) => msg.unwrap_or("An error occurred.".to_string()),
            };
            jump_message.status = false;
            jump_message.message = msg;
        }        
    }
    if jump_message.status {
        ctx.insert("jump_message", &jump_message);
        return Ok(Redirect::to("/navigation/list"));
    } else {
        ctx.insert("jump_message", &jump_message);
        return Err(super::webhotel_render(state.tera.clone(), ctx, "common/message.html"));
    }
    // let handler_name = "Message";
    // #[allow(unused_assignments)]
    // let mut message = String::from("OK");
    // let mut tpl = crate::view::MsgTemplate::default();
    // tpl.title = "添加".to_string();
    // match result {
    //     Ok(msg) => {
    //         message = msg;
    //         tpl.is_success = true;
    //     }
    //     Err(e) => {
    //         let _msg = match e.error {
    //             crate::err::AppErrorItem::Cause(err) => err.to_string(),
    //             crate::err::AppErrorItem::Message(msg) => msg.unwrap_or("发生错误".to_string()),
    //         };
    //         message = _msg;
    //         tpl.is_success = false;
    //     }
    // }
    // tpl.msg = message;
    // tpl.target_url = Some("/navigation/list".to_string());

    // render(tpl, handler_name)
}
