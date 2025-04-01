use actix_web::{
    error, get, http::header::ACCESS_CONTROL_ALLOW_METHODS, options, web, HttpResponse, Responder,
    Result,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::db_actions::selects::{find_user_by_email, find_user_by_id, find_user_by_username},
    DbPool,
};

#[derive(Serialize)]
struct DbResponse {
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

#[get("/user-id")]
pub async fn get_user_by_id(
    pool: web::Data<DbPool>,
    user_id: web::Query<UserId>,
) -> Result<HttpResponse> {
    let user = web::block(move || {
        let conn = &mut pool.get()?;

        find_user_by_id(conn, user_id.into_inner().user_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError);

    match user {
        Ok(_) => Ok(HttpResponse::Ok().json(DbResponse {
            code: 0,
            message: "Success".to_owned(),
        })),
        Err(error) => Ok(HttpResponse::Ok().json(DbResponse {
            code: 1,
            message: error.to_string(),
        })),
    }
}

#[options("/user-id")]
pub async fn user_id_options() -> impl Responder {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "GET"))
        .finish()
}

#[get("/username")]
pub async fn get_user_by_username(
    pool: web::Data<DbPool>,
    username: web::Query<Username>,
) -> Result<HttpResponse> {
    let user = web::block(move || {
        let conn = &mut pool.get()?;

        find_user_by_username(conn, &username.into_inner().username)
    })
    .await?
    .map_err(error::ErrorInternalServerError);

    match user {
        Ok(_) => Ok(HttpResponse::Ok().json(DbResponse {
            code: 0,
            message: "Success".to_owned(),
        })),
        Err(error) => Ok(HttpResponse::Ok().json(DbResponse {
            code: 1,
            message: error.to_string(),
        })),
    }
}

#[options("/username")]
pub async fn username_options() -> impl Responder {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "GET"))
        .finish()
}

#[get("/email")]
pub async fn get_user_by_email(
    pool: web::Data<DbPool>,
    email: web::Query<Email>,
) -> Result<HttpResponse> {
    let user = web::block(move || {
        let conn = &mut pool.get()?;

        find_user_by_email(conn, &email.into_inner().email)
    })
    .await?
    .map_err(error::ErrorInternalServerError);

    match user {
        Ok(_) => Ok(HttpResponse::Ok().json(DbResponse {
            code: 0,
            message: "Success".to_owned(),
        })),
        Err(error) => Ok(HttpResponse::Ok().json(DbResponse {
            code: 1,
            message: error.to_string(),
        })),
    }
}

#[options("/email")]
pub async fn email_options() -> impl Responder {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "GET"))
        .finish()
}
