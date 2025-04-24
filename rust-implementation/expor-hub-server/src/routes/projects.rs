use actix_web::{
    error::ErrorInternalServerError,
    get,
    http::header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS},
    options, post,
    web::{self, Query},
    HttpResponse, Responder, Result,
};

use crate::{
    db::db_actions::{
        inserts::{insert_project, insert_project_image},
        selects::{
            count_projects_by_user, find_project_by_id, find_projects_by_user_id,
            find_projects_by_user_id_udate_desc,
        },
    },
    routes::{NoDates, ProjectId, ServerResponse, UserId},
    DbPool,
};

use super::ProjectDetails;

#[get("/project-id")]
pub async fn get_project_by_id(
    pool: web::Data<DbPool>,
    project_id: Query<ProjectId>,
) -> Result<HttpResponse> {
    let project = web::block(move || {
        let conn = &mut pool.get()?;

        find_project_by_id(conn, project_id.into_inner().project_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match project {
        Ok(project) => Ok(HttpResponse::Ok().json(project)),
        Err(_error) => Ok(HttpResponse::Ok().json(ServerResponse {
            code: 1,
            message: "No project".to_owned(),
        })),
    }
}

#[get("/user-id")]
pub async fn get_projects_by_user_id(
    pool: web::Data<DbPool>,
    user_id: Query<UserId>,
) -> Result<HttpResponse> {
    let projects = web::block(move || {
        let conn = &mut pool.get()?;

        find_projects_by_user_id(conn, user_id.into_inner().user_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match projects {
        Ok(projects) => Ok(HttpResponse::Ok().json(projects)),
        Err(_error) => Ok(HttpResponse::Ok().json(ServerResponse {
            code: 1,
            message: "No projects".to_owned(),
        })),
    }
}

#[get("/date-updated")]
pub async fn get_projects_by_date_updated(
    pool: web::Data<DbPool>,
    user_id: Query<UserId>,
) -> Result<HttpResponse> {
    let projects = web::block(move || {
        let conn = &mut pool.get()?;

        find_projects_by_user_id_udate_desc(conn, user_id.into_inner().user_id)
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

#[get("/num-of-projects")]
pub async fn get_num_of_projects_by_user(
    pool: web::Data<DbPool>,
    user_id: web::Query<UserId>,
) -> Result<HttpResponse> {
    let result = web::block(move || {
        let conn = &mut pool.get()?;

        count_projects_by_user(conn, user_id.into_inner().user_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match result {
        Ok(num) => Ok(HttpResponse::Ok().body(num.to_string())),
        Err(error) => Ok(HttpResponse::Ok().json(ServerResponse {
            code: 1,
            message: error.to_string(),
        })),
    }
}

#[post("/create-project")]
pub async fn create_project(
    pool: web::Data<DbPool>,
    inputs: web::Json<ProjectDetails>,
) -> Result<HttpResponse> {
    let inputs = inputs.into_inner();

    let project = web::block(move || {
        let conn = &mut pool.get().unwrap();

        let result = insert_project(
            conn,
            &inputs.project_name,
            &inputs.description,
            inputs.user_id,
        );

        match result {
            Ok(result) => {
                for name in inputs.images {
                    let _ = insert_project_image(
                        conn,
                        &format!("uploads/project/{}", name),
                        inputs.user_id,
                        result.as_ref().unwrap().project_id,
                    );
                }

                result
            }
            Err(_) => None,
        }
    })
    .await?;

    match project {
        Some(project) => Ok(HttpResponse::Ok().json(project)),
        None => Ok(HttpResponse::Ok().json(ServerResponse {
            code: 1,
            message: "Failed to create project".to_owned(),
        })),
    }
}

#[options("/create-project")]
pub async fn create_project_options() -> impl Responder {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "POST"))
        .insert_header((ACCESS_CONTROL_ALLOW_HEADERS, "content-type"))
        .finish()
}
