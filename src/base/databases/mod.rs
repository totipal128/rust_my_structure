use std::env;

use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use dotenvy::dotenv;
use once_cell::sync::Lazy;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub static DB_CONNECTION:Lazy<DbPool> =Lazy::new(||{
    dotenv().ok();
    let user_env = env::var("DB_USER").unwrap_or("user".to_string());
    let pass_env = env::var("DB_PASS").unwrap_or("password".to_string());
    let host_env = env::var("DB_HOST").unwrap_or("localhost".to_string());
    let port_env = env::var("DB_PORT").unwrap_or("5432".to_string());
    let db_env = env::var("DB_NAME").unwrap_or("db".to_string());
    let databases_url = format!("postgres://{}:{}@{}:{}/{}", user_env, pass_env, host_env, port_env, db_env);

    
    let manager = ConnectionManager::<PgConnection>::new(databases_url);
    r2d2::Pool::builder()
    .build(manager)
     .expect("❌ Failed to create pool")
});