pub mod users;
pub mod navigation;
use crate::{AppState,err::AppError,Result};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
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



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct CountModel {
    pub count: i64,
}

use async_trait::async_trait;
#[async_trait]
pub trait InterfaceDB<T>
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    /// delete data by sql  
    async fn delete<'a, 'b>(state: &'a AppState, sql: &'b String) -> Result<String> {
        let pool = get_db_conn(&state);
        //let sql=format!("Delete from {} where id ={}",get_table_name(),id);
        let res = sqlx::query(&sql).execute(pool).await;
        match res {
            Ok(result) => {
                let _rows = result.rows_affected();
                if _rows == 0 {
                    let code = AppError::from_err(
                        format!("Delete operation rejected - invalid/non-existent ID").into(),
                        crate::err::AppErrorType::Database,
                    );
                    return Err(code);
                }
            }
            Err(err) => {
                let code = AppError::from_err(err.into(), crate::AppErrorType::Database);
                return Err(code);
            }
        }
        // operation redis:flush cache
        Self::remove_redis_cache(&state);

        Ok("ok".to_string())
    }
    
    /// get cache name
    fn get_cache_name() -> String {
        // let g = Model::default();
        // g.get_cache_name().to_string()
        "all_iot".to_string()
        
    }
    /// 程序获取一条数据的操作
    // #[allow(dead_code)]
    // async fn get_one_by_id<'a, 'b,'T>(
    //     state: &'a DbState,
    //     sql: &'b String,
    // ) -> std::result::Result<T, String> {
    //     let pool = get_db_conn(&state);
    //     let result = sqlx::query_as::<_, T>(&sql)
    //         .fetch_one(pool)
    //         .await
    //         .map_err(|e| format!("Error fetching from the database: {}", e))?;
    //     Ok(result)
    // }
    /// remove redis cache
    fn remove_redis_cache(state: &AppState) {
        // operation redis:flush cache
        let client = get_redis_conn(&state);
        let mut redis_conn = client.get_connection().expect("redis connect error");
        let _=redis::cmd("DEL")
            .arg(Self::get_cache_name())
            .exec(&mut redis_conn);
    }
    /// update data by sql
    async fn update<'a, 'b>(state: &'a AppState, sql: &'b String) -> Result<String> {
        let pool = get_db_conn(&state);
        let res = sqlx::query(&sql).execute(pool).await;
        match res {
            Ok(result) => {
                let _rows = result.rows_affected();
                if _rows == 0 {
                    let code = AppError::from_err(
                        format!("Cannot update record - specified ID not found.").into(),
                        crate::err::AppErrorType::Database,
                    );
                    return Err(code);
                }
            }
            Err(err) => {
                let code = AppError::from_err(err.into(), crate::AppErrorType::Database);
                return Err(code);
            }
        }
        Self::remove_redis_cache(&state);
        Ok("ok".to_string())
    }
}

