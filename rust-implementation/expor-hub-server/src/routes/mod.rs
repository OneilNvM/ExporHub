use serde::{Deserialize, Serialize};

pub mod account;
pub mod api;
pub mod favourites;
pub mod follows;
pub mod projects;
pub mod root;
pub mod users;
pub mod searches;
pub mod images;
pub mod comments;
pub mod replies;
pub mod likes;
pub mod dislikes;

#[derive(Serialize)]
struct ServerResponse {
    code: u8,
    message: String,
}

#[derive(Deserialize)]
struct UserId {
    user_id: i32,
}
#[derive(Deserialize)]
struct Username {
    username: String,
}
#[derive(Deserialize)]
struct Email {
    email: String,
}

#[derive(Deserialize)]
struct ProjectId {
    project_id: i32,
}

#[derive(Serialize)]
struct NoDates {
    message: String,
}

#[derive(Deserialize)]
struct UserAndProjectId {
    user_id: i32,
    project_id: i32,
}

#[derive(Deserialize)]
struct UserCredentials {
    username_or_email: String,
    password: String,
}

#[derive(Deserialize)]
struct UserData {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct FollowIds {
    follower: i32,
    following: i32,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String
}