use diesel::prelude::*;

use crate::errors::error::DeleteError;

pub fn delete_user_by_id(
    conn: &mut PgConnection,
    in_user_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::users::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(users.filter(user_id.eq(in_user_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_user_by_username(
    conn: &mut PgConnection,
    in_username: &str,
) -> Result<usize, DeleteError> {
    use crate::schema::users::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(users.filter(username.eq(in_username))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_project_by_id(
    conn: &mut PgConnection,
    in_project_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::projects::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(projects.filter(project_id.eq(in_project_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_project_by_name(
    conn: &mut PgConnection,
    in_name: &str,
) -> Result<usize, DeleteError> {
    use crate::schema::projects::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(projects.filter(name.eq(in_name))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_image_by_id(
    conn: &mut PgConnection,
    in_image_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::images::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(images.filter(image_id.eq(in_image_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_image_by_file_path(
    conn: &mut PgConnection,
    in_file_path: &str,
) -> Result<usize, DeleteError> {
    use crate::schema::images::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(images.filter(file_path.eq(in_file_path))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_favourite_by_id(
    conn: &mut PgConnection,
    in_favourite_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::favourites::dsl::*;
    use DeleteError::*;

    let rows_deleted =
        diesel::delete(favourites.filter(favourite_id.eq(in_favourite_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_favourite_by_ids(
    conn: &mut PgConnection,
    in_user_id: i32,
    in_project_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::favourites::dsl::*;
    use DeleteError::*;

    let rows_deleted =
        diesel::delete(favourites.filter(user_id.eq(in_user_id).and(project_id.eq(in_project_id))))
            .execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_follow_by_id(
    conn: &mut PgConnection,
    in_follow_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::follows::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(follows.filter(follow_id.eq(in_follow_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_follow_by_ids(
    conn: &mut PgConnection,
    in_follower_id: i32,
    in_following_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::follows::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(
        follows.filter(
            follower
                .eq(in_follower_id)
                .and(following.eq(in_following_id)),
        ),
    )
    .execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_comment_by_id(
    conn: &mut PgConnection,
    in_comment_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::comments::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(comments.filter(comment_id.eq(in_comment_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_reply_by_id(
    conn: &mut PgConnection,
    in_reply_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::replies::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(replies.filter(reply_id.eq(in_reply_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_thread_by_id(
    conn: &mut PgConnection,
    in_thread_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::threads::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(threads.filter(thread_id.eq(in_thread_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_thread_by_ids(
    conn: &mut PgConnection,
    in_comment_id: i32,
    in_reply_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::threads::dsl::*;
    use DeleteError::*;

    let rows_deleted =
        diesel::delete(threads.filter(comment_id.eq(in_comment_id).and(reply_id.eq(in_reply_id))))
            .execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_like_by_id(
    conn: &mut PgConnection,
    in_like_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::likes::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(likes.filter(like_id.eq(in_like_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_dislike_by_id(
    conn: &mut PgConnection,
    in_dislike_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::dislikes::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(dislikes.filter(dislike_id.eq(in_dislike_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_comment_like_by_id(
    conn: &mut PgConnection,
    in_comment_like_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::comment_likes::dsl::*;
    use DeleteError::*;

    let rows_deleted =
        diesel::delete(comment_likes.filter(id.eq(in_comment_like_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_comment_like_by_ids(
    conn: &mut PgConnection,
    in_comment_id: i32,
    in_like_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::comment_likes::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(
        comment_likes.filter(comment_id.eq(in_comment_id).and(like_id.eq(in_like_id))),
    )
    .execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_comment_dislike_by_id(
    conn: &mut PgConnection,
    in_comment_dislike_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::comment_dislikes::dsl::*;
    use DeleteError::*;

    let rows_deleted =
        diesel::delete(comment_dislikes.filter(id.eq(in_comment_dislike_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_comment_dislike_by_ids(
    conn: &mut PgConnection,
    in_comment_id: i32,
    in_dislike_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::comment_dislikes::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(
        comment_dislikes.filter(
            comment_id
                .eq(in_comment_id)
                .and(dislike_id.eq(in_dislike_id)),
        ),
    )
    .execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_reply_like_by_id(
    conn: &mut PgConnection,
    in_reply_like_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::reply_likes::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(reply_likes.filter(id.eq(in_reply_like_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_reply_like_by_ids(
    conn: &mut PgConnection,
    in_reply_id: i32,
    in_like_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::reply_likes::dsl::*;
    use DeleteError::*;

    let rows_deleted =
        diesel::delete(reply_likes.filter(reply_id.eq(in_reply_id).and(like_id.eq(in_like_id))))
            .execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_reply_dislike_by_id(
    conn: &mut PgConnection,
    in_reply_dislike_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::reply_dislikes::dsl::*;
    use DeleteError::*;

    let rows_deleted =
        diesel::delete(reply_dislikes.filter(id.eq(in_reply_dislike_id))).execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}

pub fn delete_reply_dislike_by_ids(
    conn: &mut PgConnection,
    in_reply_id: i32,
    in_dislike_id: i32,
) -> Result<usize, DeleteError> {
    use crate::schema::reply_dislikes::dsl::*;
    use DeleteError::*;

    let rows_deleted = diesel::delete(
        reply_dislikes.filter(reply_id.eq(in_reply_id).and(dislike_id.eq(in_dislike_id))),
    )
    .execute(conn);

    match rows_deleted {
        Ok(num) => {
            if num == 0 {
                Err(ZeroRowsDeleted)
            } else {
                Ok(num)
            }
        }
        Err(error) => Err(DatabaseError(error)),
    }
}
