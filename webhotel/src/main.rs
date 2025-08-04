use tower_http::{
    services::{ServeDir},
    // trace::TraceLayer,
};
use webhotel::{config,router,utils,dbstate::DbState,AppState,base_controller_middleware};
use tera::Tera;
use axum::{
    // http::StatusCode,
    // response::IntoResponse,
    // routing::{get_service},
    Extension,
    http::Request,
    body::Body,
    middleware,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_sessions::{SessionManagerLayer};
// use tower_sessions::MemoryStore; // In production, migrate to persistent storage such as Redis.
use tower_sessions_redis_store::{fred::prelude::*,RedisStore}; // In production, migrate to persistent storage such as Redis.
use std::sync::{Arc,Mutex};
use sqlx::postgres::PgPoolOptions;
// use tokio::sync::Mutex;

async fn clear_all_sessions(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    let keys: Vec<String> = pool.hkeys("*webhotel_session*").await?;

    if !keys.is_empty() {
        let _: usize = pool.del(keys).await?;
    }

    Ok(())
}
#[tokio::main]
async fn main() {
    let cfg=webhotel::new("webhotel");
    // println!("I'm lying flat.\t I'm locking down.");
    tracing::info!("I'm lying flat.\t I'm locking down in webhotel.");
    // tracing::debug!("{}---{}",&cfg.web.addr.clone(),&cfg.web.version.clone());
    let web_info: config::WebHotelInfo=config::WebHotelInfo{
        web_addr:cfg.web.addr.clone(),
        web_version:cfg.web.version.clone(),
        default: cfg.langconf.default.clone(),
        supported: cfg.langconf.supported.clone(),
    };
    
    let run_mode=cfg.web.runmode.clone();
    let website_name=std::env::var("WEBSITE_NAME").unwrap_or_else(|_| "webhotel".to_string());
    std::env::set_var("WEBSITE_VERSION", &cfg.web.version);
    if run_mode=="dev"{
       tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    website_name
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();
    }else{
        tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    website_name
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    }
    // let serve_dir = get_service(ServeDir::new("./static")).handle_error(webhotel::handle_error);
    // let css_dir = get_service(ServeDir::new("./static/css")).handle_error(webhotel::handle_error);
    // let js_dir = get_service(ServeDir::new("./static/js")).handle_error(webhotel::handle_error);
    // let images_dir = get_service(ServeDir::new("./static/images")).handle_error(webhotel::handle_error);
    let serve_dir = ServeDir::new("./static");
    let css_dir = ServeDir::new("./static/css");
    let js_dir = ServeDir::new("./static/js");
    let images_dir = ServeDir::new("./static/images");
    // init app state
    let mut tera = Tera::new("templates/**/*").unwrap();
    // 注册自定义函数
    tera.register_function("check_is_href", utils::template::CheckIsHrefFunction);
    tera.register_function("check_power", utils::template::CheckPowerFunction);
    let redis_url = format!(
        "redis://:{}@{}:{}",
        cfg.redis.password.clone(),
        cfg.redis.host.clone(),
        cfg.redis.port.clone(),
    );
    // 连接postgresql
    let db_pool = PgPoolOptions::new()
        .max_connections(cfg.db.connections)
        .connect(&cfg.db.pg).await.unwrap();
    // 连接redis
    let redis_client=redis::Client::open(redis_url.clone()).expect("Redis Database connect error");
    let app_state = AppState { tera, path_segments: Mutex::new(vec![]), db_state: DbState { conn: db_pool, redis_conn: redis_client } };

    // ‌Initialize session storage (in-memory storage, for demonstration purposes only).
    // let session_store = MemoryStore::default();
    
   
    // let pool = Pool::new(Config::new(redis_url), None, None, None, 6).unwrap();
    let pool = Pool::new(tower_sessions_redis_store::fred::prelude::Config::from_url(&redis_url).unwrap(), None, None, None, 6).unwrap();
    let redis_conn = pool.connect();
    pool.wait_for_connect().await.unwrap();

    // 清除所有 session (可选)
clear_all_sessions(&pool).await.expect("Failed to clear sessions");
    // 2. 创建 Redis 存储
    let redis_store = RedisStore::new(pool);

    // 3. 创建会话管理层
    let session_layer = SessionManagerLayer::new(redis_store)
        .with_name("webhotel_session")
        .with_secure(cfg.redis.with_secure.clone()); // 生产环境应该设为 true

    // 高级写法
    // let session_layer = SessionManagerLayer::new(redis_store)
    // .with_name("my_app_session")
    // .with_secure(true) // 生产环境启用
    // .with_max_age(Duration::hours(2)) // 2小时过期
    // .with_same_site(tower_cookies::SameSite::Lax)
    // .with_domain("example.com");

    // let session_layer = SessionManagerLayer::new(session_store);
    

    let app=router::init()
        .nest_service("/static", serve_dir.clone())
        .nest_service("/css", css_dir.clone())
        .nest_service("/js", js_dir.clone())
        .nest_service("/images", images_dir.clone())
        .fallback_service(serve_dir)
        .layer(Extension(Arc::new(web_info)))
        .layer(Extension(Arc::new(app_state.clone())))
        .layer(session_layer)
        .layer(middleware::from_fn(base_controller_middleware))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    tracing::info_span!(
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_request(move|request: &Request<Body>, _span: &tracing::Span| {
                    tracing::info!("Started {} {}", request.method(), request.uri());
                     let uri = request.uri();
                    // 1. get the full path
                    let full_path = uri.path();
                    // tracing::info!("完整路径: {}", full_path);
                    // 2. split the path into segments
                    // let path_segments: Vec<&str> = full_path.split('/').filter(|s| !s.is_empty()).collect();
                    let path_segments: Vec<String> = full_path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
                    tracing::info!("路径段: {:?}", path_segments);
                   
                    let app_state = app_state.clone();
                    let mut path_segments_lock = app_state.path_segments.lock().unwrap();
                    *path_segments_lock = path_segments;
                    tracing::info!("-----{:?}", path_segments_lock);
                })
                .on_response(|response: &axum::response::Response, latency: std::time::Duration, _span: &tracing::Span| {
                    if response.status().is_client_error() || response.status().is_server_error() {
                        tracing::error!(
                            "Error response: {} in {}ms",
                            response.status(),
                            latency.as_millis()
                        );
                    }else{
                        tracing::info!(
                            "Completed {} in {}ms",
                            response.status(),
                            latency.as_millis()
                        );
                    }
                })
        );
    tracing::info!("🌱🌎 服务监听于{}🌐🌱", &cfg.web.addr);
    let listener = tokio::net::TcpListener::bind(&cfg.web.addr).await.unwrap();
    axum::serve(listener,app).await.unwrap();
    let _= redis_conn.await.unwrap();
    // axum::Server::bind(&cfg.web.addr.parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}

