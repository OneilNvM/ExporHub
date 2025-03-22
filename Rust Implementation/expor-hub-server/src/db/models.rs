use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use crate::schema::*;
use diesel::mysql::Mysql;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Mysql))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
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

#[derive(Queryable, Selectable, Associations)]
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
    pub date_updated: Option<NaiveDateTime>
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

#[derive(Queryable, Selectable, Associations)]
#[diesel(table_name = images)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(Mysql))]
pub struct Image {
    pub image_id: i32,
    pub file_name: String,
    pub user_id: Option<i32>,
    pub project_id: Option<i32>,
    pub date_uploaded: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = images)]
pub struct NewUserImage<'a> {
    pub file_name: &'a str,
    pub user_id: i32,
    pub date_uploaded: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = images)]
pub struct NewProjectImage<'a> {
    pub file_name: &'a str,
    pub user_id: i32,
    pub project_id: i32,
    pub date_uploaded: NaiveDateTime
}

#[derive(Queryable, Selectable, Associations)]
#[diesel(table_name = favourites)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(Mysql))]
pub struct Favourite {
    pub favourite_id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub date_favourited: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = favourites)]
pub struct NewFavourite {
    pub user_id: i32,
    pub project_id: i32,
    pub date_favourited: NaiveDateTime
}

#[derive(Queryable, Selectable, Associations)]
#[diesel(table_name = follows)]
#[diesel(belongs_to(User, foreign_key = following))]
#[diesel(check_for_backend(Mysql))]
pub struct Follow {
    pub follow_id: i32,
    pub follower: i32,
    pub following: i32,
    pub date_followed: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = follows)]
pub struct NewFollow {
    pub follower: i32,
    pub following: i32,
    pub date_followed: NaiveDateTime
}

#[derive(Queryable, Selectable, Associations)]
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
    pub replies: i32
}

#[derive(Queryable, Selectable, Associations)]
#[diesel(table_name = replies)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Mysql))]
pub struct Reply {
    pub reply_id: i32,
    pub text: String,
    pub date: NaiveDate,
    pub user_id: i32
}

#[derive(Queryable, Selectable, Associations)]
#[diesel(table_name = threads)]
#[diesel(belongs_to(Comment))]
#[diesel(belongs_to(Reply))]
#[diesel(check_for_backend(Mysql))]
pub struct Thread {
    pub thread_id: i32,
    pub comment_id: i32,
    pub reply_id: i32
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = likes)]
#[diesel(check_for_backend(Mysql))]
pub struct Like {
    pub like_id: i32,
    pub date_liked: NaiveDate
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = dislikes)]
#[diesel(check_for_backend(Mysql))]
pub struct Dislike {
    pub dislike_id: i32,
    pub date_disliked: NaiveDate
}