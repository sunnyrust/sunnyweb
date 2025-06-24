pub mod user;
use crate::{dbstate::DbState,AppState};
// use serde::{de::DeserializeOwned, Deserialize, Serialize};
#[allow(dead_code)]
/// 取得PostgreSQL的conn
fn get_db_conn<'a>(state: &'a AppState) -> &'a sqlx::PgPool {
    &state.db_state.conn
}

#[allow(dead_code)]
/// 取得redis的conn
fn get_redis_conn<'a>(state: &'a AppState) -> &'a redis::Client {
    &state.db_state.redis_conn
}
