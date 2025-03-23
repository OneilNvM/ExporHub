use diesel::prelude::*;

use crate::errors::error::DeleteError;

pub fn delete_user_by_id(conn: &mut MysqlConnection, in_user_id: i32) -> Result<usize, DeleteError> {
    use crate::schema::users::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(users.filter(user_id.eq(in_user_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                return Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        },
        Err(error) => Err(DatabaseError(error))
    }
}

pub fn delete_user_by_username(conn: &mut MysqlConnection, in_username: &str) -> Result<usize, DeleteError> {
    use crate::schema::users::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(users.filter(username.eq(in_username))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                return Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        },
        Err(error) => Err(DatabaseError(error))
    }
}

pub fn delete_project_by_id(conn: &mut MysqlConnection, in_project_id: i32) -> Result<usize, DeleteError> {
    use crate::schema::projects::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(projects.filter(project_id.eq(in_project_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                return Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        },
        Err(error) => Err(DatabaseError(error))
    }
}

pub fn delete_project_by_name(conn: &mut MysqlConnection, in_name: &str) -> Result<usize, DeleteError> {
    use crate::schema::projects::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(projects.filter(name.eq(in_name))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                return Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        },
        Err(error) => Err(DatabaseError(error))
    }
}

pub fn delete_image_by_id(conn: &mut MysqlConnection, in_image_id: i32) -> Result<usize, DeleteError> {
    use crate::schema::images::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(images.filter(image_id.eq(in_image_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                return Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        },
        Err(error) => Err(DatabaseError(error))
    }
}

pub fn delete_image_by_file_name(conn: &mut MysqlConnection, in_file_name: &str) -> Result<usize, DeleteError> {
    use crate::schema::images::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(images.filter(file_name.eq(in_file_name))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                return Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        },
        Err(error) => Err(DatabaseError(error))
    }
}