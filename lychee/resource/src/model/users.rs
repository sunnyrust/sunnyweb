/// user model
use redis::{Commands, };
use serde::{Deserialize, Serialize};
use super::{get_db_conn, get_redis_conn};
use crate::{AppError,Result,AppState};
use sunny_derive_trait::*;
use sunny_derive::*;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow,PgCurdStruct)]
#[TableName = "lychee_rbac.users"]
#[CacheName = "all_user"]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
    pub email:String,
    pub username:String,
    pub description:String,
    pub is_active:bool,
}
impl Default for Model {
    fn default() -> Self {
        Model {
            id: 0,
            name: String::new(),
            password_hash: String::new(),
            email: String::new(),
            username: String::new(),
            description: String::new(),
            is_active: false,
        }
    }
}


/// 判断名字是不是在数据库中存在
async fn have_name<'a,'b,'c>(state: &'a AppState,table_name:&'b String,name:&'c String) ->Result<bool>{
    let pool = get_db_conn(&state);
    let sql=format!("SELECT count(1) as count from {} where name ='{}'",table_name,name);
    let mut b=false;  
    let parent_rows = sqlx::query_as::<_, super::CountModel>(&sql)
        .fetch_one(pool)
        .await;
     match parent_rows {
             Ok(result) => {
                 if result.count!=0{
                     b=true;
                 }
             },
             Err(err) => {
                 tracing::error!("----{}---",err);
             }
     }
     Ok(b)
 }

#[allow(dead_code)]
pub async fn insert_one<'a,'b,'c,'d>(state: &'a AppState,sql:&'b String,table_name:&'c String,name:&'d  String) -> Result<String> {
    
    let pool = get_db_conn(&state);
    let b=have_name(state,table_name,name).await.unwrap();
    if b{
        let code = AppError::from_err(format!("name:{}:The nickname already exists.",name).into(),crate::AppErrorType::Database);
        return Err(code);
    }
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
    // remove redis cache
    // operation redis:flush cache
    remove_redis_cache(&state);
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
    let mut vec_model:Vec<Model>=vec![];
    if !b_have_key{
        let pool = get_db_conn(&state);
        let rows = sqlx::query_as::<_, Model>(&sql)
            .fetch_all(pool)
            .await
            .unwrap();
        vec_model=rows.clone();
        // insert redis cache
        let strm:String=serde_json::to_string(&rows).unwrap();
        let _:()=redis_conn.set(get_cache_name(),  strm).unwrap();
    }else{
        vec_model=serde_json::from_str(&result).unwrap();
    }
    Ok(vec_model)
}
#[allow(dead_code)]
/// get one data by id
pub async fn get_one<'a,'b>(state: &'a AppState,sql:&'b String) -> std::result::Result<Model,String>  {
    let pool = get_db_conn(&state);
    let result = sqlx::query_as::<_, Model>(&sql)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| format!("Error fetching from the database: {}", e))?;
    Ok(result)
}

#[allow(dead_code)]
pub async fn update<'a,'b>(state: &'a AppState,sql:&'b String) -> Result<String> {
    let pool = get_db_conn(&state);
    let res=sqlx::query(&sql)
    .execute(pool)
    .await;
    match res {
        Ok(result) => {
            let _rows=result.rows_affected();
            if _rows==0{
                let code = AppError::from_err(format!("Error: ID not found in the database. Update aborted.").into(),crate::AppErrorType::Database);
                return Err(code);
            }
        },
        Err(err) => {
            let code = AppError::from_err(err.into(),crate::AppErrorType::Database);
            return Err(code);
        }
    }
    // 操作redis 清除缓存
    let client=get_redis_conn(&state);
    let mut redis_conn = client.get_connection().expect("redis connect error");
    let _=redis::cmd("DEL").arg(get_cache_name()).exec(&mut redis_conn);
    Ok("ok".to_string())
}
/// 取得cache名字
fn get_cache_name()->String{
    let model=Model::default();
    model.get_cache_name().to_string()
}
/// 删除redis缓存中的数据
fn remove_redis_cache(state: &AppState){
    // 操作redis 清除缓存
    let client=get_redis_conn(&state);
    let mut redis_conn = client.get_connection().expect("redis connect error");
    let _ = redis::cmd("DEL").arg(get_cache_name()).exec(&mut redis_conn);
}
