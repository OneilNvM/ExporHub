use std::collections::HashMap;

use crate::{db::models::*, errors::error::UpdateError};
use chrono::NaiveDateTime;
use diesel::prelude::*;

pub fn update_user(
    conn: &mut MysqlConnection,
    in_user_id: i32,
    values_map: HashMap<&str, Option<&str>>,
) -> Result<Option<User>, anyhow::Error> {
    use crate::schema::users::dsl::*;

    let user = conn.transaction(|conn| {
        for (key, value) in values_map {
            match key {
                "username" => {
                    diesel::update(users.find(in_user_id))
                        .set(username.eq(value.unwrap().to_owned()))
                        .execute(conn)?;
                }
                "email" => {
                    diesel::update(users.find(in_user_id))
                        .set(email.eq(value.unwrap().to_owned()))
                        .execute(conn)?;
                }
                "password" => {
                    diesel::update(users.find(in_user_id))
                        .set(password.eq(value.unwrap().to_owned()))
                        .execute(conn)?;
                }
                "bio" => {
                    diesel::update(users.find(in_user_id))
                        .set(bio.eq(value.unwrap().to_owned()))
                        .execute(conn)?;
                }
                "profile_img" => {
                    diesel::update(users.find(in_user_id))
                        .set(profile_img.eq(value.unwrap().to_owned()))
                        .execute(conn)?;
                }
                "followers" => {
                    let num: i32 = value.unwrap().parse().unwrap();

                    diesel::update(users.find(in_user_id))
                        .set(followers.eq(num))
                        .execute(conn)?;
                }
                _ => eprintln!("No column named {key}"),
            }
        }

        users
            .find(in_user_id)
            .select(User::as_select())
            .first(conn)
            .optional()
    });

    match user {
        Ok(user) => Ok(user),
        Err(error) => Err(error.into()),
    }
}

pub fn update_project(
    conn: &mut MysqlConnection,
    in_project_id: i32,
    values_map: HashMap<&str, Option<&str>>,
) -> Result<Option<Project>, UpdateError> {
    use crate::schema::projects::dsl::*;
    use UpdateError::*;

    let project = conn.transaction(|conn| {
        for (key, value) in values_map {
            match key {
                "name" => {
                    diesel::update(projects.find(in_project_id))
                        .set(name.eq(value.unwrap().to_owned()))
                        .execute(conn)?;
                }
                "description" => {
                    diesel::update(projects.find(in_project_id))
                        .set(description.eq(value.unwrap().to_owned()))
                        .execute(conn)?;
                }
                "favourites" => {
                    let num: i32 = value.unwrap().parse().unwrap();

                    diesel::update(projects.find(in_project_id))
                        .set(favourites.eq(num))
                        .execute(conn)?;
                }
                "date_updated" => {
                    let date =
                        NaiveDateTime::parse_from_str(value.unwrap(), "%Y-%m-%d %H:%M:%S%.6f")
                            .unwrap_or_default();

                    if date == NaiveDateTime::default() {
                        println!("{:?}", value.unwrap());
                        break;
                    }

                    diesel::update(projects.find(in_project_id))
                        .set(date_updated.eq(date))
                        .execute(conn)?;
                }
                _ => eprintln!("No column named {key}"),
            }
        }

        projects
            .find(in_project_id)
            .select(Project::as_select())
            .first(conn)
            .optional()
    });

    match project {
        Ok(project) => Ok(project),
        Err(error) => Err(DatabaseError(error)),
    }
}
