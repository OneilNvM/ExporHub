use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{self, Query},
    HttpResponse, Result,
};
use serde::{Deserialize, Serialize};

use crate::{db::db_actions::selects::find_project_by_user_id_udate_desc, DbPool};

#[derive(Deserialize)]
struct UserId {
    user_id: i32,
}

#[derive(Serialize)]
struct NoDates {
    message: String,
}

#[get("/date-updated")]
pub async fn get_projects_by_date_updated(
    pool: web::Data<DbPool>,
    user_id: Query<UserId>,
) -> Result<HttpResponse> {
    let projects = web::block(move || {
        let conn = &mut pool.get()?;

        find_project_by_user_id_udate_desc(conn, user_id.into_inner().user_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match projects {
        Ok(projects) => Ok(HttpResponse::Ok().json(projects)),
        Err(_error) => Ok(HttpResponse::Ok().json(NoDates {
            message: "No projects".to_owned(),
        })),
    }
}
