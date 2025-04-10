use serde::{Deserialize, Serialize};

pub mod account;
pub mod api;
pub mod favourite;
pub mod follow;
pub mod projects;
pub mod root;
pub mod users;

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
