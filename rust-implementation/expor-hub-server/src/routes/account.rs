use actix_web::{
    error, post, web, HttpResponse, Result,
};
use sha2::{Digest, Sha256};

use crate::{
    db::db_actions::{
        connection::establish_connection,
        inserts::insert_user,
        selects::{find_user_by_email, find_user_by_username},
    },
    errors::error::LoginError,
    routes::{ServerResponse, UserCredentials, UserData},
    DbPool,
};

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    credentials: web::Json<UserCredentials>,
) -> Result<HttpResponse> {
    let credentials = credentials.into_inner();

    if credentials.username_or_email.contains("@") {
        let user = web::block(move || {
            let conn = &mut pool.get()?;

            find_user_by_email(conn, &credentials.username_or_email)
        })
        .await?
        .map_err(error::ErrorInternalServerError);

        match user {
            Ok(user) => {
                let mut hasher = Sha256::new();
                hasher.update(credentials.password.as_bytes());
                let result = hasher.finalize();

                let hex = hex::encode(result);

                if hex == user.password {
                    Ok(HttpResponse::Ok().json(user))
                } else {
                    Ok(HttpResponse::BadRequest().json(ServerResponse {
                        code: 400,
                        message: LoginError::InvalidCredentials.to_string(),
                    }))
                }
            }
            Err(error) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
                code: 500,
                message: error.to_string(),
            })),
        }
    } else {
        let user = web::block(move || {
            let conn = &mut establish_connection();

            find_user_by_username(conn, &credentials.username_or_email)
        })
        .await?
        .map_err(error::ErrorInternalServerError);

        match user {
            Ok(user) => {
                let mut hasher = Sha256::new();
                hasher.update(credentials.password.as_bytes());
                let result = hasher.finalize();

                let hex = hex::encode(result);

                if hex == user.password {
                    Ok(HttpResponse::Ok().json(user))
                } else {
                    Ok(HttpResponse::BadRequest().json(ServerResponse {
                        code: 400,
                        message: LoginError::InvalidCredentials.to_string(),
                    }))
                }
            }
            Err(error) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
                code: 500,
                message: error.to_string(),
            })),
        }
    }
}

#[post("/create-account")]
pub async fn create_account(
    pool: web::Data<DbPool>,
    inputs: web::Json<UserData>,
) -> Result<HttpResponse> {
    let inputs = inputs.into_inner();

    let user = web::block(move || {
        let conn = &mut pool.get().unwrap();

        insert_user(conn, &inputs.username, &inputs.email, &inputs.password)
    })
    .await?
    .map_err(error::ErrorInternalServerError);

    match user {
        Ok(user) => Ok(HttpResponse::Ok().json(user.unwrap())),
        Err(error) => Ok(HttpResponse::InternalServerError().json(ServerResponse {
            code: 500,
            message: error.to_string(),
        })),
    }
}
