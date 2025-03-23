use super::models::*;
use crate::errors::error::{AccountCreationError::{self, *}, LoginError};
use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use dotenvy::dotenv;
use sha2::{Digest, Sha256};
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("'DATABASE_URL' must be set in the .env file.");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to MySQL database url {}", database_url))
}

pub fn find_user_by_id(conn: &mut MysqlConnection, in_id: i32) -> Result<Option<User>, LoginError> {
    use crate::schema::users::dsl::*;

    let results: Result<Option<User>, diesel::result::Error> = users
        .filter(user_id.eq(in_id))
        .select(User::as_select())
        .get_result(conn)
        .optional();

    match results {
        Ok(user) => Ok(user),
        Err(error) => Err(LoginError::DatabaseError(error))
    }
}

pub fn find_user_by_email(conn: &mut MysqlConnection, in_email: &str) -> Result<Option<User>, LoginError> {
    use crate::schema::users::dsl::*;

    let results = users
        .select(User::as_select())
        .filter(email.eq(in_email))
        .get_result(conn)
        .optional();

    match results {
        Ok(user) => Ok(user),
        Err(error) => Err(LoginError::DatabaseError(error))
    }
}

pub fn find_user_by_username(conn: &mut MysqlConnection, in_username: &str) -> Result<Option<User>, LoginError> {
    use crate::schema::users::dsl::*;

    let results: Result<Option<User>, diesel::result::Error> = users
        .filter(username.eq(in_username))
        .select(User::as_select())
        .get_result(conn)
        .optional();

    match results {
        Ok(user) => Ok(user),
        Err(error) => Err(LoginError::DatabaseError(error))
    }
}

pub fn insert_user(
    conn: &mut MysqlConnection,
    username: &str,
    email: &str,
    password: &str,
) -> Result<Option<User>, AccountCreationError> {
    use crate::schema::users;

    if username.len() < 3 {
        return Err(InvalidUsername(username.to_owned()));
    } else if username.contains("@") {
        return Err(InvalidUsername(username.to_owned()));
    } else if !email.contains("@") {
        return Err(InvalidEmail(email.to_owned()));
    } else if password.len() < 8 {
        return Err(InvalidPassword);
    } else {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let result = hasher.finalize();

        let hex = hex::encode(result);

        let new_user = NewUser {
            username,
            email,
            password: &hex,
            followers: 0,
            date_created: NaiveDateTime::new(Local::now().date_naive(), Local::now().time()),
        };

        let user = conn.transaction(|conn| {
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(conn)?;

            users::table
                .order(users::user_id.desc())
                .select(User::as_select())
                .first(conn)
                .optional()
        });

        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(DatabaseError(e))
        }
    }
}


