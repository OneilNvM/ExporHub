use diesel::prelude::*;
use crate::db::models::*;

pub fn find_user_by_id(conn: &mut MysqlConnection, in_id: i32) -> Result<User, anyhow::Error> {
    use crate::schema::users::dsl::*;

    let results = users
        .filter(user_id.eq(in_id))
        .select(User::as_select())
        .get_result(conn);

    match results {
        Ok(user) => Ok(user),
        Err(error) => Err(error.into())
    }
}

pub fn find_user_by_email(conn: &mut MysqlConnection, in_email: &str) -> Result<User, anyhow::Error> {
    use crate::schema::users::dsl::*;

    let results = users
        .select(User::as_select())
        .filter(email.eq(in_email))
        .get_result(conn);

    match results {
        Ok(user) => Ok(user),
        Err(error) => Err(error.into())
    }
}

pub fn find_user_by_username(conn: &mut MysqlConnection, in_username: &str) -> Result<User, anyhow::Error> {
    use crate::schema::users::dsl::*;

    let results = users
        .filter(username.eq(in_username))
        .select(User::as_select())
        .get_result(conn);

    match results {
        Ok(user) => Ok(user),
        Err(error) => Err(error.into())
    }
}

pub fn find_project_by_id(conn: &mut MysqlConnection, in_id: i32) -> Result<Project, anyhow::Error> {
    use crate::schema::projects::dsl::*;

    let project = projects
        .select(Project::as_select())
        .filter(project_id.eq(in_id))
        .get_result(conn);

    match project {
        Ok(project) => Ok(project),
        Err(error) => Err(error.into())
    }
}

pub fn find_project_by_name(conn: &mut MysqlConnection, in_name: &str) -> Result<Project, anyhow::Error> {
    use crate::schema::projects::dsl::*;

    let project = projects
        .select(Project::as_select())
        .filter(name.eq(in_name))
        .get_result(conn);

    match project {
        Ok(project) => Ok(project),
        Err(error) => Err(error.into())
    }
}