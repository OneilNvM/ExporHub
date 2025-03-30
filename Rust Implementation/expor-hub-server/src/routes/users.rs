use actix_web::{
    get, options, web, HttpRequest, HttpResponse, Responder, Result
};
use serde::Serialize;

use crate::{db::db_actions::selects::find_user_by_username, DbPool};

#[derive(Serialize)]
struct DbResponse {
    code: u8,
    message: String,
}

#[get("/{username}")]
pub async fn get_user_by_username(
    pool: web::Data<DbPool>,
    username: web::Path<String>,
) -> Result<HttpResponse> {
    let user = web::block(move || {
        let conn = &mut pool.get()?;

        find_user_by_username(conn, &username)
    })
    .await?;

    match user {
        Ok(_) => Ok(HttpResponse::Ok().json(DbResponse {
            code: 0,
            message: "Success".to_owned(),
        })),
        Err(error) => Ok(HttpResponse::InternalServerError().json(DbResponse {
            code: 1,
            message: error.to_string(),
        })),
    }
}

#[options("/{username}")]
pub async fn username_options(req: HttpRequest) -> impl Responder {
    println!("{:#?}", req.headers());
    HttpResponse::NoContent()
        .finish()
}
