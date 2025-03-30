use std::env;

use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::basic::BasicAuth;
use db::models::*;
use diesel::{r2d2, MysqlConnection};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};

pub mod db;
pub mod errors;
pub mod routes;
pub mod schema;

#[cfg(test)]
mod tests;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;

#[derive(Serialize, Deserialize)]
pub enum TableTypes {
    Users(Vec<User>),
    Projects(Vec<Project>),
    Images(Vec<Image>),
    Follows(Vec<Follow>),
    Favourites(Vec<Favourite>),
    Comments(Vec<Comment>),
    Replies(Vec<Reply>),
    Threads(Vec<Thread>),
    Likes(Vec<Like>),
    Dislikes(Vec<Dislike>),
    CommentLikes(Vec<CommentLike>),
    CommentDislikes(Vec<CommentDislike>),
    ReplyLikes(Vec<ReplyLike>),
    ReplyDislikes(Vec<ReplyDislike>),
}

pub fn initialize_db_pool() -> DbPool {
    dotenv().unwrap();

    let conn_spec = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager: r2d2::ConnectionManager<MysqlConnection> = r2d2::ConnectionManager::new(conn_spec);

    r2d2::Pool::builder().build(manager).unwrap()
}

pub async fn validate_auth(req: ServiceRequest, creds: BasicAuth) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    if creds.user_id() == "OneilNvM" && creds.password().unwrap() == "nether1215" {
        Ok(req)
    } else {
        Err((actix_web::error::ErrorUnauthorized("Request Denied"), req))
    }
}
