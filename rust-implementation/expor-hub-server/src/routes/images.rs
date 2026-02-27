use actix_web::{
    error::ErrorInternalServerError,
    get, post, web, HttpResponse, Result,
};

use crate::{
    db::db_actions::{
        inserts::insert_profile_image,
        selects::{find_profile_image, find_project_images},
    },
    routes::{ProfileImageUpload, ServerResponse, UserAndProjectId, UserId},
    DbPool,
};

#[get("/profile-image")]
pub async fn get_profile_image(
    pool: web::Data<DbPool>,
    user_id: web::Query<UserId>,
) -> Result<HttpResponse> {
    let user_id = user_id.into_inner().user_id;

    let image = web::block(move || {
        let conn = &mut pool.get()?;

        find_profile_image(conn, user_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match image {
        Ok(image) => Ok(HttpResponse::Ok().json(image)),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
            code: 500,
            message: "Image not found".to_owned(),
        })),
    }
}

#[get("/project-images")]
pub async fn get_project_images(
    pool: web::Data<DbPool>,
    inputs: web::Query<UserAndProjectId>,
) -> Result<HttpResponse> {
    let inputs = inputs.into_inner();

    let images = web::block(move || {
        let conn = &mut pool.get()?;

        find_project_images(conn, inputs.user_id, inputs.project_id)
    })
    .await?
    .map_err(ErrorInternalServerError);

    match images {
        Ok(images) => Ok(HttpResponse::Ok().json(images)),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
            code: 500,
            message: "Project images not found".to_owned(),
        })),
    }
}

#[post("/upload")]
pub async fn upload_profile_image(
    pool: web::Data<DbPool>,
    inputs: web::Json<ProfileImageUpload>,
) -> Result<HttpResponse> {
    let inputs = inputs.into_inner();

    let result = web::block(move || {
        let conn = &mut pool.get()?;

        insert_profile_image(
            conn,
            &format!("uploads/profile/{}", inputs.image),
            inputs.user_id,
        )
    })
    .await?
    .map_err(ErrorInternalServerError);

    match result {
        Ok(result) => match result {
            Some(image) => Ok(HttpResponse::Ok().json(image)),
            None => Ok(HttpResponse::InternalServerError().json(ServerResponse {
                code: 500,
                message: "Failed to upload image path".to_owned(),
            })),
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
            code: 500,
            message: "Failed to upload image path".to_owned(),
        })),
    }
}
