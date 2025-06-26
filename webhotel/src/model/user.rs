/// user model
use redis::{Commands, };
use serde::{Deserialize, Serialize};
use super::{get_db_conn, get_redis_conn};
use crate::{AppError,dbstate::DbState,Result,AppState};
use std::collections::BTreeMap;
use std::cell::{RefCell,};
use std::collections::HashMap;
use sunny_derive_trait::*;
use sunny_derive::*;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow,PgCurdStruct)]
#[TableName = "lychee_rbac.user"]
#[CacheName = "all_user"]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
    pub email:String,
    pub is_parent:bool,
}
impl Default for Model {
    fn default() -> Self {
        Model {
            id: 0,
            name: String::new(),
            password_hash: String::new(),
            email: String::new(),
            is_parent: false,
        }
    }
}

#[allow(dead_code)]
pub async fn insert_one<'a,'b>(state: &'a AppState,sql:&'b String) -> Result<String> {
    let pool = get_db_conn(&state);
    let res=sqlx::query(&sql)
    .execute(pool)
    .await;
    match res {
        Ok(result) => {
            let _rows=result.rows_affected();
            if _rows==0{
                let code = AppError::from_err(format!("Insert Error").into(),crate::AppErrorType::Database);
                return Err(code);
            }
        },
        Err(err) => {
            let code = AppError::from_err(err.into(),crate::AppErrorType::Database);
            return Err(code);
        }
    }
    // clear redis cache
    let client=get_redis_conn(&state);
    let mut redis_conn = client.get_connection().expect("redis connect error");
    let _ = redis::cmd("DEL").arg(get_cache_name()).exec(&mut redis_conn);
    Ok("ok".to_string())
}

#[allow(dead_code)]
pub async fn get_all<'a,'b>(state: &'a AppState,sql:&'b String) -> Result<Vec<Model>> {
    // 操作redis
    let client=get_redis_conn(&state);
    let mut redis_conn = client.get_connection().expect("redis connect error");
    let mut b_have_key=false;  //have cache?
    let rv:redis::RedisResult<String> = redis_conn.get(get_cache_name());//get cache
    let result =match rv {
        Ok(data) => {
            b_have_key=true;
            data
        },
        Err(_err) => {
            "".to_string()
        }
    };
    #[allow(unused_assignments)]
    let mut vec_emotion:Vec<Model>=vec![];
    if !b_have_key{
        let pool = get_db_conn(&state);
        let rows = sqlx::query_as::<_, Model>(&sql)
            .fetch_all(pool)
            .await
            .unwrap();
        vec_emotion=rows.clone();
        // insert redis cache
        let strm:String=serde_json::to_string(&rows).unwrap();
        let _:()=redis_conn.set(get_cache_name(),  strm).unwrap();
    }else{
        vec_emotion=serde_json::from_str(&result).unwrap();
    }
    Ok(vec_emotion)
}
#[allow(dead_code)]
/// get one data by id
pub async fn get_one_by_id<'a,'b>(state: &'a AppState,sql:&'b String) -> std::result::Result<Model,String>  {
    let pool = get_db_conn(&state);
    let result = sqlx::query_as::<_, Model>(&sql)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| format!("Error fetching from the database: {}", e))?;
    Ok(result)
}


fn get_cache_name()->String{
    let g=Model::default();
    g.get_cache_name().to_string()
}


