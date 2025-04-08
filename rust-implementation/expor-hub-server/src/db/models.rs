use crate::schema::*;
use chrono::{Local, NaiveDate, NaiveDateTime};
use diesel::mysql::Mysql;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Mysql))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub profile_img: Option<String>,
    pub followers: i32,
    pub date_created: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub followers: i32,
    pub date_created: NaiveDateTime,
}

impl<'a> NewUser<'a> {
    pub fn new(username: &'a str, email: &'a str, password: &'a str) -> Self {
        Self {
            username,
            email,
            password,
            followers: 0,
            date_created: NaiveDateTime::new(Local::now().date_naive(), Local::now().time()),
        }
    }
}

#[derive(Queryable, Selectable, Associations, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = projects)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Mysql))]
pub struct Project {
    pub project_id: i32,
    pub name: String,
    pub description: String,
    pub favourites: i32,
    pub user_id: i32,
    pub date_created: NaiveDateTime,
    pub date_updated: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub favourites: i32,
    pub user_id: i32,
    pub date_created: NaiveDateTime,
}

impl<'a> NewProject<'a> {
    pub fn new(name: &'a str, description: &'a str, user_id: i32) -> Self {
        Self {
            name,
            description,
            favourites: 0,
            user_id,
            date_created: NaiveDateTime::new(Local::now().date_naive(), Local::now().time()),
        }
    }
}

#[derive(Queryable, Selectable, Associations, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = images)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(Mysql))]
pub struct Image {
    pub image_id: i32,
    pub file_path: String,
    pub user_id: Option<i32>,
    pub project_id: Option<i32>,
    pub date_uploaded: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = images)]
pub struct NewUserImage<'a> {
    pub file_path: &'a str,
    pub user_id: i32,
    pub date_uploaded: NaiveDateTime,
}

impl<'a> NewUserImage<'a> {
    pub fn new(file_path: &'a str, user_id: i32) -> Self {
        Self {
            file_path,
            user_id,
            date_uploaded: NaiveDateTime::new(Local::now().date_naive(), Local::now().time()),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = images)]
pub struct NewProjectImage<'a> {
    pub file_path: &'a str,
    pub user_id: i32,
    pub project_id: i32,
    pub date_uploaded: NaiveDateTime,
}

impl<'a> NewProjectImage<'a> {
    pub fn new(file_path: &'a str, user_id: i32, project_id: i32) -> Self {
        Self {
            file_path,
            user_id,
            project_id,
            date_uploaded: NaiveDateTime::new(Local::now().date_naive(), Local::now().time()),
        }
    }
}

#[derive(Queryable, Selectable, Associations, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = favourites)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(Mysql))]
pub struct Favourite {
    pub favourite_id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub date_favourited: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = favourites)]
pub struct NewFavourite {
    pub user_id: i32,
    pub project_id: i32,
    pub date_favourited: NaiveDateTime,
}

impl NewFavourite {
    pub fn new(user_id: i32, project_id: i32) -> Self {
        Self {
            user_id,
            project_id,
            date_favourited: NaiveDateTime::new(Local::now().date_naive(), Local::now().time()),
        }
    }
}

#[derive(Queryable, Selectable, Associations, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = follows)]
#[diesel(belongs_to(User, foreign_key = following))]
#[diesel(check_for_backend(Mysql))]
pub struct Follow {
    pub follow_id: i32,
    pub follower: i32,
    pub following: i32,
    pub date_followed: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = follows)]
pub struct NewFollow {
    pub follower: i32,
    pub following: i32,
    pub date_followed: NaiveDateTime,
}

impl NewFollow {
    pub fn new(follower: i32, following: i32) -> Self {
        Self {
            follower,
            following,
            date_followed: NaiveDateTime::new(Local::now().date_naive(), Local::now().time()),
        }
    }
}

#[derive(Queryable, Selectable, Associations, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = comments)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(Mysql))]
pub struct Comment {
    pub comment_id: i32,
    pub text: String,
    pub date: NaiveDate,
    pub user_id: i32,
    pub project_id: i32,
    pub replies: i32,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment<'a> {
    pub text: &'a str,
    pub date: NaiveDate,
    pub user_id: i32,
    pub project_id: i32,
    pub replies: i32,
}

impl<'a> NewComment<'a> {
    pub fn new(text: &'a str, user_id: i32, project_id: i32) -> Self {
        Self {
            text,
            date: Local::now().date_naive(),
            user_id,
            project_id,
            replies: 0,
        }
    }
}

#[derive(Queryable, Selectable, Associations, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = replies)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Mysql))]
pub struct Reply {
    pub reply_id: i32,
    pub text: String,
    pub date: NaiveDate,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = replies)]
pub struct NewReply<'a> {
    pub text: &'a str,
    pub date: NaiveDate,
    pub user_id: i32,
}

impl<'a> NewReply<'a> {
    pub fn new(text: &'a str, user_id: i32) -> Self {
        Self {
            text,
            date: Local::now().date_naive(),
            user_id,
        }
    }
}

#[derive(Queryable, Selectable, Associations, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = threads)]
#[diesel(belongs_to(Comment))]
#[diesel(belongs_to(Reply))]
#[diesel(check_for_backend(Mysql))]
pub struct Thread {
    pub thread_id: i32,
    pub comment_id: i32,
    pub reply_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = threads)]
pub struct NewThread {
    pub comment_id: i32,
    pub reply_id: i32,
}

impl NewThread {
    pub fn new(comment_id: i32, reply_id: i32) -> Self {
        Self {
            comment_id,
            reply_id,
        }
    }
}

#[derive(Queryable, Selectable, Associations, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = likes)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Mysql))]
pub struct Like {
    pub like_id: i32,
    pub user_id: i32,
    pub date_liked: NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = likes)]
pub struct NewLike {
    pub user_id: i32,
    pub date_liked: NaiveDate,
}

#[derive(Queryable, Selectable, Associations, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = dislikes)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Mysql))]
pub struct Dislike {
    pub dislike_id: i32,
    pub user_id: i32,
    pub date_disliked: NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = dislikes)]
pub struct NewDislike {
    pub user_id: i32,
    pub date_disliked: NaiveDate,
}

#[derive(Queryable, Selectable, Associations, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = comment_likes)]
#[diesel(belongs_to(Comment))]
#[diesel(belongs_to(Like))]
#[diesel(check_for_backend(Mysql))]
pub struct CommentLike {
    pub id: i32,
    pub comment_id: i32,
    pub like_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = comment_likes)]
pub struct NewCommentLike {
    pub comment_id: i32,
    pub like_id: i32,
}

#[derive(Queryable, Selectable, Associations, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = comment_dislikes)]
#[diesel(belongs_to(Comment))]
#[diesel(belongs_to(Dislike))]
#[diesel(check_for_backend(Mysql))]
pub struct CommentDislike {
    pub id: i32,
    pub comment_id: i32,
    pub dislike_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = comment_dislikes)]
pub struct NewCommentDislike {
    pub comment_id: i32,
    pub dislike_id: i32,
}

#[derive(Queryable, Selectable, Associations, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = reply_likes)]
#[diesel(belongs_to(Reply))]
#[diesel(belongs_to(Like))]
#[diesel(check_for_backend(Mysql))]
pub struct ReplyLike {
    pub id: i32,
    pub reply_id: i32,
    pub like_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = reply_likes)]
pub struct NewReplyLike {
    pub reply_id: i32,
    pub like_id: i32,
}

#[derive(Queryable, Selectable, Associations, Debug, Default, Serialize, Deserialize)]
#[diesel(table_name = reply_dislikes)]
#[diesel(belongs_to(Reply))]
#[diesel(belongs_to(Dislike))]
#[diesel(check_for_backend(Mysql))]
pub struct ReplyDislike {
    pub id: i32,
    pub reply_id: i32,
    pub dislike_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = reply_dislikes)]
pub struct NewReplyDislike {
    pub reply_id: i32,
    pub dislike_id: i32,
}
