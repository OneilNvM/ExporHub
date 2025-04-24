use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("'DATABASE_URL' must be set in the .env file.");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to MySQL database url {}", database_url))
}
