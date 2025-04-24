use actix_web::{error::ErrorInternalServerError, get, web, HttpResponse, Result};

use crate::{db::db_actions::selects::find_project_comments, routes::ServerResponse, DbPool};

use super::ProjectId;

#[get("/project-comments")]
pub async fn get_project_comments(
    pool: web::Data<DbPool>,
    inputs: web::Query<ProjectId>,
) -> Result<HttpResponse> {
    let inputs = inputs.into_inner();

    let comments = web::block(move || {
        let conn = &mut pool.get()?;

        find_project_comments(conn, inputs.project_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match comments {
        Ok(comments) => Ok(HttpResponse::Ok().json(comments)),
        Err(_) => Ok(HttpResponse::Ok().json(ServerResponse {
            code: 1,
            message: "Failed to find comments".to_owned(),
        })),
    }
}
