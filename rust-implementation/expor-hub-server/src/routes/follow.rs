use actix_web::{
    error::ErrorInternalServerError, get, web, HttpResponse, Result
};

use crate::{db::db_actions::selects::find_user_followings, routes::{ServerResponse, UserId}, DbPool};

#[get("/user-id")]
pub async fn get_user_followings(pool: web::Data<DbPool>, user_id: web::Query<UserId>) -> Result<HttpResponse> {
    let followings = web::block(move || {
        let conn = &mut pool.get()?;

        find_user_followings(conn, user_id.into_inner().user_id)
    }).await?.map_err(ErrorInternalServerError);

    match followings {
        Ok(results) => Ok(HttpResponse::Ok().json(results)),
        Err(_error) => Ok(HttpResponse::Ok().json(ServerResponse {
            code: 1, message: "No Followings".to_owned()
        }))
    }
}