use actix_web::{error::ErrorInternalServerError, get, web, HttpResponse, Result};

use crate::{
    db::db_actions::selects::{count_comment_dislikes, count_reply_dislikes, find_user_dislikes},
    routes::{CommentId, ReplyId, ServerResponse},
    DbPool,
};

use super::UserId;

#[get("/user-dislikes")]
pub async fn get_user_dislikes(
    pool: web::Data<DbPool>,
    inputs: web::Query<UserId>,
) -> Result<HttpResponse> {
    let inputs = inputs.into_inner();

    let dislikes = web::block(move || {
        let conn = &mut pool.get()?;

        find_user_dislikes(conn, inputs.user_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match dislikes {
        Ok(dislikes) => Ok(HttpResponse::Ok().json(dislikes)),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
            code: 500,
            message: "Failed to retrieve user's dislikes".to_owned(),
        })),
    }
}

#[get("/comment-dislikes")]
pub async fn get_comment_dislikes(
    pool: web::Data<DbPool>,
    inputs: web::Query<CommentId>,
) -> Result<HttpResponse> {
    let inputs = inputs.into_inner();

    let dislikes = web::block(move || {
        let conn = &mut pool.get()?;

        count_comment_dislikes(conn, inputs.comment_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match dislikes {
        Ok(dislikes) => Ok(HttpResponse::Ok().json(dislikes)),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
            code: 500,
            message: "Failed to retrieve comment dislikes".to_owned(),
        })),
    }
}

#[get("/reply-dislikes")]
pub async fn get_reply_dislikes(
    pool: web::Data<DbPool>,
    inputs: web::Query<ReplyId>,
) -> Result<HttpResponse> {
    let inputs = inputs.into_inner();

    let dislikes = web::block(move || {
        let conn = &mut pool.get()?;

        count_reply_dislikes(conn, inputs.reply_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match dislikes {
        Ok(dislikes) => Ok(HttpResponse::Ok().json(dislikes)),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
            code: 500,
            message: "Failed to retrieve reply dislikes".to_owned(),
        })),
    }
}
