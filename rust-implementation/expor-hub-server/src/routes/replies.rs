use actix_web::{error::ErrorInternalServerError, get, web, HttpResponse, Result};

use crate::{
    db::db_actions::selects::find_project_replies,
    routes::{CommentId, ServerResponse},
    DbPool,
};

#[get("/thread-replies")]
pub async fn get_comment_replies(
    pool: web::Data<DbPool>,
    inputs: web::Query<CommentId>,
) -> Result<HttpResponse> {
    let inputs = inputs.into_inner();

    let replies = web::block(move || {
        let conn = &mut pool.get()?;

        find_project_replies(conn, inputs.comment_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match replies {
        Ok(replies) => Ok(HttpResponse::Ok().json(replies)),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
            code: 500,
            message: "Failed to retrieve replies".to_owned(),
        })),
    }
}
