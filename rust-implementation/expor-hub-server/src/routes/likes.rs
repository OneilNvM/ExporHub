use actix_web::{error::ErrorInternalServerError, get, web, HttpResponse, Result};

use crate::{
    db::db_actions::selects::{count_comment_likes, count_reply_likes, find_user_likes},
    routes::{CommentId, ReplyId, ServerResponse, UserId},
    DbPool,
};

#[get("/user-likes")]
pub async fn get_user_likes(
    pool: web::Data<DbPool>,
    inputs: web::Query<UserId>,
) -> Result<HttpResponse> {
    let inputs = inputs.into_inner();

    let likes = web::block(move || {
        let conn = &mut pool.get()?;

        find_user_likes(conn, inputs.user_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match likes {
        Ok(likes) => Ok(HttpResponse::Ok().json(likes)),
        Err(_) => Ok(HttpResponse::Ok().json(ServerResponse {
            code: 1,
            message: "Failed to retrieve user's likes".to_owned(),
        })),
    }
}

#[get("/comment-likes")]
pub async fn get_comment_likes(
    pool: web::Data<DbPool>,
    inputs: web::Query<CommentId>,
) -> Result<HttpResponse> {
    let inputs = inputs.into_inner();

    let likes = web::block(move || {
        let conn = &mut pool.get()?;

        count_comment_likes(conn, inputs.comment_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match likes {
        Ok(likes) => Ok(HttpResponse::Ok().content_type("text/plain").json(likes)),
        Err(_) => Ok(HttpResponse::Ok().json(ServerResponse {
            code: 1,
            message: "Failed to retrieve comment likes".to_owned(),
        })),
    }
}

#[get("/reply-likes")]
pub async fn get_reply_likes(
    pool: web::Data<DbPool>,
    inputs: web::Query<ReplyId>,
) -> Result<HttpResponse> {
    let inputs = inputs.into_inner();

    let likes = web::block(move || {
        let conn = &mut pool.get()?;

        count_reply_likes(conn, inputs.reply_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match likes {
        Ok(likes) => Ok(HttpResponse::Ok().json(likes)),
        Err(_) => Ok(HttpResponse::Ok().json(ServerResponse {
            code: 1,
            message: "Failed to retrieve reply likes".to_owned(),
        })),
    }
}
