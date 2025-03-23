use chrono::{Local, NaiveDateTime};
use sha2::{Sha256, Digest};
use diesel::prelude::*;

use crate::errors::error::{AccountCreationError, ProjectCreationError};
use crate::db::models::*;

pub fn insert_user(
    conn: &mut MysqlConnection,
    username: &str,
    email: &str,
    password: &str,
) -> Result<Option<User>, AccountCreationError> {
    use crate::schema::users;
    use AccountCreationError::*;

    if username.len() < 3 {
        return Err(InvalidUsername(username.to_owned()));
    } else if username.contains("@") {
        return Err(InvalidUsername(username.to_owned()));
    } else if !email.contains("@") {
        return Err(InvalidEmail(email.to_owned()));
    } else if password.len() < 8 {
        return Err(InvalidPassword);
    } else {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let result = hasher.finalize();

        let hex = hex::encode(result);

        let new_user = NewUser {
            username,
            email,
            password: &hex,
            followers: 0,
            date_created: NaiveDateTime::new(Local::now().date_naive(), Local::now().time()),
        };

        let user = conn.transaction(|conn| {
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(conn)?;

            users::table
                .order(users::user_id.desc())
                .select(User::as_select())
                .first(conn)
                .optional()
        });

        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(DatabaseError(e))
        }
    }
}

pub fn insert_project(conn: &mut MysqlConnection, proj_name: &str, description: &str, user_id: i32) -> Result<Option<Project>, ProjectCreationError> {
    use crate::schema::projects;
    use ProjectCreationError::*;

    if proj_name.len() < 3 {
        return Err(InvalidProjectName(proj_name.to_owned()))
    } else if description.len() < 3 {
        return Err(InvalidDescription(description.to_owned()))
    } else {
        let new_project = NewProject {
            name: proj_name,
            description,
            favourites: 0,
            user_id,
            date_created: NaiveDateTime::new(Local::now().date_naive(), Local::now().time())
        };

        let project = conn.transaction(|conn| {
            diesel::insert_into(projects::table)
                .values(new_project)
                .execute(conn)?;

            projects::table
                .order(projects::project_id.desc())
                .select(Project::as_select())
                .first(conn)
                .optional()
        });

        match project {
            Ok(project) => Ok(project),
            Err(error) => Err(DatabaseError(error))
        }
    }
}