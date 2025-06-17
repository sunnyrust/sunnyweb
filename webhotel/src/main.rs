use tower_http::{
    services::{ServeDir},
    // trace::TraceLayer,
};
use webhotel::{config,router,AppState};
use tera::Tera;
use axum::{
    // http::StatusCode,
    // response::IntoResponse,
    // routing::{get_service},
    Extension,
};
use tower_sessions::{SessionManagerLayer};
use tower_sessions::MemoryStore; // In production, migrate to persistent storage such as Redis.
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let cfg=webhotel::new("webhotel");
    // println!("I'm lying flat.\t I'm locking down.");
    tracing::info!("I'm lying flat.\t I'm locking down in webhotel.");
    // tracing::debug!("{}---{}",&cfg.web.addr.clone(),&cfg.web.version.clone());
    let web_info=config::WebHotelInfo{
        web_addr:cfg.web.addr.clone(),
        web_version:cfg.web.version.clone(),
    };
    // let serve_dir = get_service(ServeDir::new("./static")).handle_error(webhotel::handle_error);
    // let css_dir = get_service(ServeDir::new("./static/css")).handle_error(webhotel::handle_error);
    // let js_dir = get_service(ServeDir::new("./static/js")).handle_error(webhotel::handle_error);
    // let images_dir = get_service(ServeDir::new("./static/images")).handle_error(webhotel::handle_error);
    let serve_dir = ServeDir::new("./static");
    let css_dir = ServeDir::new("./static/css");
    let js_dir = ServeDir::new("./static/js");
    let images_dir = ServeDir::new("./static/images");
    // init app state
    let tera = Tera::new("templates/**/*").unwrap();
    let app_state = AppState { tera };

    // ‌Initialize session storage (in-memory storage, for demonstration purposes only).
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);
    let app=router::init()
        .nest_service("/static", serve_dir.clone())
        .nest_service("/css", css_dir.clone())
        .nest_service("/js", js_dir.clone())
        .nest_service("/images", images_dir.clone())
        .fallback_service(serve_dir)
        .layer(Extension(Arc::new(web_info)))
        .layer(Extension(Arc::new(app_state)))
        .layer(session_layer);
    tracing::info!("🌱🌎 服务监听于{}🌐🌱", &cfg.web.addr);
    let listener = tokio::net::TcpListener::bind(&cfg.web.addr).await.unwrap();
    axum::serve(listener,app).await.unwrap();
    // axum::Server::bind(&cfg.web.addr.parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}

