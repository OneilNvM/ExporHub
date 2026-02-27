use crate::db::models::*;
use crate::errors::error::{
    AccountCreationError, CommentCreationError, ProjectCreationError, ReplyCreationError,
};
use chrono::Local;
use diesel::prelude::*;
use sha2::{Digest, Sha256};

pub fn insert_user(
    conn: &mut PgConnection,
    in_username: &str,
    in_email: &str,
    in_password: &str,
) -> Result<Option<User>, AccountCreationError> {
    use crate::schema::users;
    use AccountCreationError::*;

    if in_username.len() < 3 || in_username.contains("@") {
        Err(InvalidUsername(in_username.to_owned()))
    } else if !in_email.contains("@") {
        Err(InvalidEmail(in_email.to_owned()))
    } else if in_password.len() < 8 {
        Err(InvalidPassword)
    } else {
        let mut hasher = Sha256::new();
        hasher.update(in_password.as_bytes());
        let result = hasher.finalize();

        let hex = hex::encode(result);

        let new_user = NewUser::new(in_username, in_email, &hex);

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
            Err(e) => Err(DatabaseError(e)),
        }
    }
}

pub fn insert_project(
    conn: &mut PgConnection,
    in_name: &str,
    in_description: &str,
    in_user_id: i32,
) -> Result<Option<Project>, ProjectCreationError> {
    use crate::schema::projects;
    use ProjectCreationError::*;

    if in_name.len() < 3 {
        Err(InvalidProjectName(in_name.to_owned()))
    } else if in_description.len() < 3 {
        Err(InvalidDescription(in_description.to_owned()))
    } else {
        let new_project = NewProject::new(in_name, in_description, in_user_id);

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
            Err(error) => Err(DatabaseError(error)),
        }
    }
}

pub fn insert_profile_image(
    conn: &mut PgConnection,
    in_file_path: &str,
    in_user_id: i32,
) -> Result<Option<Image>, anyhow::Error> {
    use crate::schema::images;

    let new_image = NewUserImage::new(in_file_path, in_user_id);

    let image = conn.transaction(|conn| {
        diesel::insert_into(images::table)
            .values(&new_image)
            .execute(conn)?;

        images::table
            .order(images::image_id.desc())
            .select(Image::as_select())
            .first(conn)
            .optional()
    });

    match image {
        Ok(image) => Ok(image),
        Err(error) => Err(error.into()),
    }
}

pub fn insert_project_image(
    conn: &mut PgConnection,
    in_file_path: &str,
    in_user_id: i32,
    in_project_id: i32,
) -> Result<Option<Image>, anyhow::Error> {
    use crate::schema::images;

    let new_image = NewProjectImage::new(in_file_path, in_user_id, in_project_id);

    let image = conn.transaction(|conn| {
        diesel::insert_into(images::table)
            .values(&new_image)
            .execute(conn)?;

        images::table
            .order(images::image_id.desc())
            .select(Image::as_select())
            .first(conn)
            .optional()
    });

    match image {
        Ok(image) => Ok(image),
        Err(error) => Err(error.into()),
    }
}

pub fn insert_favourite(
    conn: &mut PgConnection,
    in_user_id: i32,
    in_project_id: i32,
) -> Result<Option<Favourite>, anyhow::Error> {
    use crate::schema::favourites;

    let new_favourite = NewFavourite::new(in_user_id, in_project_id);

    let favourite = conn.transaction(|conn| {
        diesel::insert_into(favourites::table)
            .values(&new_favourite)
            .execute(conn)?;

        favourites::table
            .order(favourites::favourite_id.desc())
            .select(Favourite::as_select())
            .first(conn)
            .optional()
    });

    match favourite {
        Ok(favourite) => Ok(favourite),
        Err(error) => Err(error.into()),
    }
}

pub fn insert_follow(
    conn: &mut PgConnection,
    in_follower: i32,
    in_following: i32,
) -> Result<Option<Follow>, anyhow::Error> {
    use crate::schema::follows;

    let new_follow = NewFollow::new(in_follower, in_following);

    let follow = conn.transaction(|conn| {
        diesel::insert_into(follows::table)
            .values(&new_follow)
            .execute(conn)?;

        follows::table
            .order(follows::follow_id.desc())
            .select(Follow::as_select())
            .first(conn)
            .optional()
    });

    match follow {
        Ok(follow) => Ok(follow),
        Err(error) => Err(error.into()),
    }
}

pub fn insert_comment(
    conn: &mut PgConnection,
    in_text: &str,
    in_user_id: i32,
    in_project_id: i32,
) -> Result<Option<Comment>, CommentCreationError> {
    use crate::schema::comments;
    use CommentCreationError::*;

    if in_text.is_empty() {
        Err(EmptyComment)
    } else {
        let new_comment = NewComment::new(in_text, in_user_id, in_project_id);

        let comment = conn.transaction(|conn| {
            diesel::insert_into(comments::table)
                .values(&new_comment)
                .execute(conn)?;

            comments::table
                .order(comments::comment_id.desc())
                .select(Comment::as_select())
                .first(conn)
                .optional()
        });

        match comment {
            Ok(comment) => Ok(comment),
            Err(error) => Err(DatabaseError(error)),
        }
    }
}

pub fn insert_reply(
    conn: &mut PgConnection,
    in_text: &str,
    in_user_id: i32,
) -> Result<Option<Reply>, ReplyCreationError> {
    use crate::schema::replies;
    use ReplyCreationError::*;

    if in_text.is_empty() {
        Err(EmptyReply)
    } else {
        let new_reply = NewReply::new(in_text, in_user_id);

        let reply = conn.transaction(|conn| {
            diesel::insert_into(replies::table)
                .values(&new_reply)
                .execute(conn)?;

            replies::table
                .order(replies::reply_id.desc())
                .select(Reply::as_select())
                .first(conn)
                .optional()
        });

        match reply {
            Ok(reply) => Ok(reply),
            Err(error) => Err(DatabaseError(error)),
        }
    }
}

pub fn insert_thread(
    conn: &mut PgConnection,
    in_comment_id: i32,
    in_reply_id: i32,
) -> Result<Option<Thread>, anyhow::Error> {
    use crate::schema::threads;

    let new_thread = NewThread::new(in_comment_id, in_reply_id);

    let thread = conn.transaction(|conn| {
        diesel::insert_into(threads::table)
            .values(&new_thread)
            .execute(conn)?;

        threads::table
            .order(threads::thread_id.desc())
            .select(Thread::as_select())
            .first(conn)
            .optional()
    });

    match thread {
        Ok(thread) => Ok(thread),
        Err(error) => Err(error.into()),
    }
}

pub fn insert_like(
    conn: &mut PgConnection,
    in_user_id: i32,
) -> Result<Option<Like>, anyhow::Error> {
    use crate::schema::likes;

    let new_like = NewLike {
        user_id: in_user_id,
        date_liked: Local::now().date_naive(),
    };

    let like = conn.transaction(|conn| {
        diesel::insert_into(likes::table)
            .values(&new_like)
            .execute(conn)?;

        likes::table
            .order(likes::like_id.desc())
            .select(Like::as_select())
            .first(conn)
            .optional()
    });

    match like {
        Ok(like) => Ok(like),
        Err(error) => Err(error.into()),
    }
}

pub fn insert_dislike(
    conn: &mut PgConnection,
    in_user_id: i32,
) -> Result<Option<Dislike>, anyhow::Error> {
    use crate::schema::dislikes;

    let new_dislike = NewDislike {
        user_id: in_user_id,
        date_disliked: Local::now().date_naive(),
    };

    let dislike = conn.transaction(|conn| {
        diesel::insert_into(dislikes::table)
            .values(&new_dislike)
            .execute(conn)?;

        dislikes::table
            .order(dislikes::dislike_id.desc())
            .select(Dislike::as_select())
            .first(conn)
            .optional()
    });

    match dislike {
        Ok(dislike) => Ok(dislike),
        Err(error) => Err(error.into()),
    }
}

pub fn insert_comment_like(
    conn: &mut PgConnection,
    in_comment_id: i32,
    in_like_id: i32,
) -> Result<Option<CommentLike>, anyhow::Error> {
    use crate::schema::comment_likes;

    let new_comment_like = NewCommentLike {
        comment_id: in_comment_id,
        like_id: in_like_id,
    };

    let comment_like = conn.transaction(|conn| {
        diesel::insert_into(comment_likes::table)
            .values(&new_comment_like)
            .execute(conn)?;

        comment_likes::table
            .order(comment_likes::id.desc())
            .select(CommentLike::as_select())
            .first(conn)
            .optional()
    });

    match comment_like {
        Ok(comment_like) => Ok(comment_like),
        Err(error) => Err(error.into()),
    }
}

pub fn insert_comment_dislike(
    conn: &mut PgConnection,
    in_comment_id: i32,
    in_dislike_id: i32,
) -> Result<Option<CommentDislike>, anyhow::Error> {
    use crate::schema::comment_dislikes;

    let new_comment_dislike = NewCommentDislike {
        comment_id: in_comment_id,
        dislike_id: in_dislike_id,
    };

    let comment_dislike = conn.transaction(|conn| {
        diesel::insert_into(comment_dislikes::table)
            .values(&new_comment_dislike)
            .execute(conn)?;

        comment_dislikes::table
            .order(comment_dislikes::id.desc())
            .select(CommentDislike::as_select())
            .first(conn)
            .optional()
    });

    match comment_dislike {
        Ok(comment_dislike) => Ok(comment_dislike),
        Err(error) => Err(error.into()),
    }
}

pub fn insert_reply_like(
    conn: &mut PgConnection,
    in_reply_id: i32,
    in_like_id: i32,
) -> Result<Option<ReplyLike>, anyhow::Error> {
    use crate::schema::reply_likes;

    let new_reply_like = NewReplyLike {
        reply_id: in_reply_id,
        like_id: in_like_id,
    };

    let reply_like = conn.transaction(|conn| {
        diesel::insert_into(reply_likes::table)
            .values(&new_reply_like)
            .execute(conn)?;

        reply_likes::table
            .order(reply_likes::id.desc())
            .select(ReplyLike::as_select())
            .first(conn)
            .optional()
    });

    match reply_like {
        Ok(reply_like) => Ok(reply_like),
        Err(error) => Err(error.into()),
    }
}

pub fn insert_reply_dislike(
    conn: &mut PgConnection,
    in_reply_id: i32,
    in_dislike_id: i32,
) -> Result<Option<ReplyDislike>, anyhow::Error> {
    use crate::schema::reply_dislikes;

    let new_reply_dislike = NewReplyDislike {
        reply_id: in_reply_id,
        dislike_id: in_dislike_id,
    };

    let reply_dislike = conn.transaction(|conn| {
        diesel::insert_into(reply_dislikes::table)
            .values(&new_reply_dislike)
            .execute(conn)?;

        reply_dislikes::table
            .order(reply_dislikes::id.desc())
            .select(ReplyDislike::as_select())
            .first(conn)
            .optional()
    });

    match reply_dislike {
        Ok(reply_dislike) => Ok(reply_dislike),
        Err(error) => Err(error.into()),
    }
}
