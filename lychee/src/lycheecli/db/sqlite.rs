use sqlx::{ SqlitePool};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use anyhow::Result;
use std::str::FromStr;
use crate::lycheecli::utils::PasswordHasher;
use std::thread;
use std::time;
///  SQLite initial function
pub async  fn init(name:String) -> Result<()> {
    // set sqlite options
    let conn_options = SqliteConnectOptions::from_str(&format!("sqlite://{}", name))?
        .create_if_missing(true);  // if db not exist, create it
    // create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(conn_options)
        .await?;
    // create tables
    create_tables(&pool).await?;
    println!("Database setup completed successfully!");
    let millis = time::Duration::from_millis(300);
    thread::sleep(millis);
    // seed initial data
    seed_initial_data(&pool).await?;
    println!("Initial data seeded successfully!");
    // close the pool
    pool.close().await;
    Ok(())
}

async fn create_tables(pool: &SqlitePool) -> Result<()> {
    // create transaction
    let mut tx = pool.begin().await?;
    
    // create users table
    sqlx::query(
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT, --id
            username TEXT NOT NULL UNIQUE, --username
            password_hash TEXT NOT NULL    --password
        )
        "#
    )
    .execute(&mut *tx)
    .await?;
 
    
    // 提交事务
    tx.commit().await?;
    
    println!("Tables created successfully!");
    
    Ok(())
}

async fn seed_initial_data(pool: &SqlitePool) -> Result<()> {
    let password_hash = PasswordHasher::new().hash("sunny9807"); // 使用密码哈希函数生成密码哈希
    // 插入示例用户
    sqlx::query(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ('admin', ?)
        "#,
    )
    .bind(password_hash)
    .execute(pool)
    .await?;
    // 插入更多示例数据
    

    
    println!("Initial data seeded successfully!");
    
    Ok(())
}