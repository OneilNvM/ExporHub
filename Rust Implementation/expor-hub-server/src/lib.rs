use std::env;

use diesel::{r2d2, MysqlConnection};
use dotenvy::dotenv;

pub mod db;
pub mod errors;
pub mod routes;
pub mod schema;
#[cfg(test)]
mod tests;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;

pub fn initialize_db_pool() -> DbPool {
    dotenv().unwrap();

    let conn_spec = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager: r2d2::ConnectionManager<MysqlConnection> = r2d2::ConnectionManager::new(conn_spec);

    r2d2::Pool::builder().build(manager).unwrap()
}
