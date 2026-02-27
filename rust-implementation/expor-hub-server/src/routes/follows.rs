use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

use actix_web::{error::ErrorInternalServerError, get, post, web, HttpResponse, Result};

use crate::{
    db::db_actions::{
        deletes::delete_follow_by_ids,
        inserts::insert_follow,
        selects::{find_follow_by_ids, find_user_by_id, find_user_followings},
        updates::update_user,
    },
    routes::{ServerResponse, UserId},
    DbPool,
};

use super::FollowIds;

#[get("/user-id")]
pub async fn get_user_followings(
    pool: web::Data<DbPool>,
    user_id: web::Query<UserId>,
) -> Result<HttpResponse> {
    let followings = web::block(move || {
        let conn = &mut pool.get()?;

        find_user_followings(conn, user_id.into_inner().user_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match followings {
        Ok(results) => Ok(HttpResponse::Ok().json(results)),
        Err(_error) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
            code: 500,
            message: "No Followings".to_owned(),
        })),
    }
}

#[post("/unfollow")]
pub async fn unfollow(pool: web::Data<DbPool>, ids: web::Query<FollowIds>) -> Result<HttpResponse> {
    let ids = ids.into_inner();

    let result = web::block(move || {
        let conn = &mut pool.get().unwrap();

        let user = find_user_by_id(conn, ids.following).unwrap();

        let mut values: HashMap<&str, Option<&str>> = HashMap::new();

        let binding = user.followers.sub(1).to_string();
        values.insert("followers", Some(&binding));

        let _ = update_user(conn, ids.following, values).map_err(ErrorInternalServerError);

        delete_follow_by_ids(conn, ids.follower, ids.following)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match result {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(error) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
            code: 500,
            message: error.to_string(),
        })),
    }
}

#[post("/new")]
pub async fn new_follow(
    pool: web::Data<DbPool>,
    ids: web::Query<FollowIds>,
) -> Result<HttpResponse> {
    let ids = ids.into_inner();

    let follow = web::block(move || {
        let conn = &mut pool.get()?;

        let result = insert_follow(conn, ids.follower, ids.following);

        let user = find_user_by_id(conn, ids.following).unwrap();

        let mut values: HashMap<&str, Option<&str>> = HashMap::new();

        let binding = user.followers.add(1).to_string();
        values.insert("followers", Some(&binding));

        let _ = update_user(conn, ids.following, values);

        result
    })
    .await?
    .map_err(ErrorInternalServerError);

    match follow {
        Ok(follow) => Ok(HttpResponse::Ok().json(follow.unwrap())),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
            code: 500,
            message: "Failed to follow".to_owned(),
        })),
    }
}

#[get("/unique-follow")]
pub async fn get_follow_by_ids(
    pool: web::Data<DbPool>,
    ids: web::Query<FollowIds>,
) -> Result<HttpResponse> {
    let ids = ids.into_inner();

    let follow = web::block(move || {
        let conn = &mut pool.get()?;

        find_follow_by_ids(conn, ids.follower, ids.following)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match follow {
        Ok(follow) => Ok(HttpResponse::Ok().json(follow)),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
            code: 500,
            message: "Failed to find follow".to_owned(),
        })),
    }
}
