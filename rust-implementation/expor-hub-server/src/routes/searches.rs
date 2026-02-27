use actix_web::{error::ErrorInternalServerError, get, web, HttpResponse, Result};

use crate::{db::db_actions::selects::find_search_results, DbPool};

use super::{SearchQuery, ServerResponse};

#[get("/query")]
pub async fn process_search_query(
    pool: web::Data<DbPool>,
    in_query: web::Query<SearchQuery>,
) -> Result<HttpResponse> {
    let in_query = in_query.into_inner();

    let results = web::block(move || {
        let conn = &mut pool.get()?;

        find_search_results(conn, in_query.q)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match results {
        Ok(results) => Ok(HttpResponse::Ok().json(results)),
        Err(error) => Ok(HttpResponse::BadRequest().json(ServerResponse {
            code: 400,
            message: error.to_string(),
        })),
    }
}
