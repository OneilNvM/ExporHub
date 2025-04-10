use std::{collections::HashMap, ops::{Add, Sub}};

use actix_web::{
    error::ErrorInternalServerError, get, post, web, HttpResponse, Result
};

use crate::{db::db_actions::{deletes::delete_favourite_by_ids, inserts::insert_favourite, selects::{find_favourite_by_ids, find_favourites_by_user_id, find_project_by_id}, updates::update_project}, routes::{ServerResponse, UserAndProjectId, UserId}, DbPool};

#[get("/user-id")]
pub async fn get_favourites_by_user_id(pool: web::Data<DbPool>, user_id: web::Query<UserId>) -> Result<HttpResponse> {
    let favourites = web::block(move || {
        let conn = &mut pool.get()?;

        find_favourites_by_user_id(conn, user_id.into_inner().user_id)
    }).await?.map_err(ErrorInternalServerError);

    match favourites {
        Ok(favourites) => Ok(HttpResponse::Ok().json(favourites)),
        Err(_error) => Ok(HttpResponse::Ok().json(ServerResponse {
            code: 1, message: "No Favourites".to_owned()
        }))
    }
}

#[post("/unfavourite")]
pub async fn unfavourite_project(pool: web::Data<DbPool>, ids: web::Query<UserAndProjectId>) -> Result<HttpResponse> {
    let ids = ids.into_inner();

    let result = web::block(move || {
        let conn = &mut pool.get().unwrap();

        let mut values: HashMap<&str, Option<&str>> = HashMap::new();

        let project = find_project_by_id(conn, ids.project_id).unwrap();

        let binding = project.favourites.sub(1).to_string();
        values.insert("favourites", Some(&binding));

        let _ = update_project(conn, ids.project_id, values).map_err(ErrorInternalServerError);

        delete_favourite_by_ids(conn, ids.user_id, ids.project_id)
    }).await?.map_err(ErrorInternalServerError);

    match result {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(error) => Ok(HttpResponse::Ok().json(ServerResponse {
            code: 1, message: error.to_string()
        }))
    }
}

#[get("/u-p-id")]
pub async fn get_favourite_by_ids(pool: web::Data<DbPool>, ids: web::Query<UserAndProjectId>) -> Result<HttpResponse> {
    let ids = ids.into_inner();

    let favourite = web::block(move || {
        let conn = &mut pool.get()?;

        find_favourite_by_ids(conn, ids.user_id, ids.project_id)
    }).await?.map_err(ErrorInternalServerError);

    match favourite {
        Ok(favourite) => Ok(HttpResponse::Ok().json(favourite)),
        Err(_error) => Ok(HttpResponse::Ok().json(ServerResponse {
            code: 1, message: "No Favourite".to_owned()
        }))
    }
}

#[post("/new")]
pub async fn create_new_favourite(pool: web::Data<DbPool>, ids: web::Query<UserAndProjectId>) -> Result<HttpResponse> {
    let ids = ids.into_inner();

    let favourite = web::block(move || {
        let conn = &mut pool.get()?;

        let project = find_project_by_id(conn, ids.project_id).unwrap();

        let mut values: HashMap<&str, Option<&str>> = HashMap::new();

        let binding = project.favourites.add(1).to_string();

        values.insert("favourites", Some(&binding));

        let favourite = insert_favourite(conn, ids.user_id, ids.project_id);

        let _ = update_project(conn, ids.project_id, values);

        favourite
    }).await?.map_err(ErrorInternalServerError);

    match favourite {
        Ok(favourite) => Ok(HttpResponse::Ok().json(favourite.unwrap())),
        Err(_error) => Ok(HttpResponse::Ok().json(ServerResponse {
            code: 1, message: "Failed to favourite".to_owned()
        }))
    }
}