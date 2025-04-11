use actix_web::{
    error, get, http::header::ACCESS_CONTROL_ALLOW_METHODS, options, web, HttpResponse, Responder,
    Result,
};

use crate::{
    db::db_actions::selects::{find_user_by_email, find_user_by_id, find_user_by_username}, routes::{Email, ServerResponse, UserId, Username}, DbPool
};

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
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(error) => Ok(HttpResponse::Ok().json(ServerResponse {
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
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(error) => Ok(HttpResponse::Ok().json(ServerResponse {
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
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(error) => Ok(HttpResponse::Ok().json(ServerResponse {
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
