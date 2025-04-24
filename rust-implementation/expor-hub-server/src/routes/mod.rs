use serde::{Deserialize, Serialize};

pub mod account;
pub mod api;
pub mod comments;
pub mod dislikes;
pub mod favourites;
pub mod follows;
pub mod images;
pub mod likes;
pub mod projects;
pub mod replies;
pub mod root;
pub mod searches;
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

#[derive(Deserialize)]
struct FollowIds {
    follower: i32,
    following: i32,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

#[derive(Deserialize)]
struct ProjectDetails {
    project_name: String,
    description: String,
    user_id: i32,
    images: Vec<String>,
}

#[derive(Deserialize)]
struct ProfileImageUpload {
    user_id: i32,
    image: String,
}

#[derive(Deserialize)]
struct CommentId {
    comment_id: i32,
}

#[derive(Deserialize)]
struct ReplyId {
    reply_id: i32,
}
