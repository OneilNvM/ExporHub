use actix_web::{get, web, HttpResponse, Result};

use crate::{
    db::{
        db_actions::selects::{
            find_comment_dislikes, find_comment_likes, find_comments, find_dislikes,
            find_favourites, find_follows, find_images, find_likes, find_projects, find_replies,
            find_reply_dislikes, find_reply_likes, find_threads, find_users,
        },
        models::*,
    },
    DbPool,
    TableTypes::*,
};

#[get("/")]
pub async fn all_tables(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let rows = web::block(move || {
        let conn = &mut pool.get().unwrap();

        vec![
            Users(find_users(conn).unwrap_or_else(|_| vec![User::default()])),
            Projects(find_projects(conn).unwrap_or_else(|_| vec![Project::default()])),
            Images(find_images(conn).unwrap_or_else(|_| vec![Image::default()])),
            Follows(find_follows(conn).unwrap_or_else(|_| vec![Follow::default()])),
            Favourites(find_favourites(conn).unwrap_or_else(|_| vec![Favourite::default()])),
            Comments(find_comments(conn).unwrap_or_else(|_| vec![Comment::default()])),
            Replies(find_replies(conn).unwrap_or_else(|_| vec![Reply::default()])),
            Threads(find_threads(conn).unwrap_or_else(|_| vec![Thread::default()])),
            Likes(find_likes(conn).unwrap_or_else(|_| vec![Like::default()])),
            Dislikes(find_dislikes(conn).unwrap_or_else(|_| vec![Dislike::default()])),
            CommentLikes(find_comment_likes(conn).unwrap_or_else(|_| vec![CommentLike::default()])),
            CommentDislikes(
                find_comment_dislikes(conn).unwrap_or_else(|_| vec![CommentDislike::default()]),
            ),
            ReplyLikes(find_reply_likes(conn).unwrap_or_else(|_| vec![ReplyLike::default()])),
            ReplyDislikes(
                find_reply_dislikes(conn).unwrap_or_else(|_| vec![ReplyDislike::default()]),
            ),
        ]
    })
    .await?;

    Ok(HttpResponse::Ok().json(rows))
}

#[get("/users")]
pub async fn show_users(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let users = web::block(move || {
        let conn = &mut pool.get()?;

        find_users(conn)
    })
    .await?;

    match users {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}

#[get("/projects")]
pub async fn show_projects(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let projects = web::block(move || {
        let conn = &mut pool.get()?;

        find_projects(conn)
    })
    .await?;

    match projects {
        Ok(projects) => Ok(HttpResponse::Ok().json(projects)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}

#[get("/images")]
pub async fn show_images(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let images = web::block(move || {
        let conn = &mut pool.get()?;

        find_images(conn)
    })
    .await?;

    match images {
        Ok(images) => Ok(HttpResponse::Ok().json(images)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}

#[get("/follows")]
pub async fn show_follows(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let follows = web::block(move || {
        let conn = &mut pool.get()?;

        find_follows(conn)
    })
    .await?;

    match follows {
        Ok(follows) => Ok(HttpResponse::Ok().json(follows)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}

#[get("/favourites")]
pub async fn show_favourites(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let favourites = web::block(move || {
        let conn = &mut pool.get()?;

        find_favourites(conn)
    })
    .await?;

    match favourites {
        Ok(favourites) => Ok(HttpResponse::Ok().json(favourites)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}

#[get("/comments")]
pub async fn show_comments(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let comments = web::block(move || {
        let conn = &mut pool.get()?;

        find_comments(conn)
    })
    .await?;

    match comments {
        Ok(comments) => Ok(HttpResponse::Ok().json(comments)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}

#[get("/replies")]
pub async fn show_replies(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let replies = web::block(move || {
        let conn = &mut pool.get()?;

        find_replies(conn)
    })
    .await?;

    match replies {
        Ok(replies) => Ok(HttpResponse::Ok().json(replies)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}

#[get("/threads")]
pub async fn show_threads(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let threads = web::block(move || {
        let conn = &mut pool.get()?;

        find_threads(conn)
    })
    .await?;

    match threads {
        Ok(threads) => Ok(HttpResponse::Ok().json(threads)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}

#[get("/likes")]
pub async fn show_likes(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let likes = web::block(move || {
        let conn = &mut pool.get()?;

        find_likes(conn)
    })
    .await?;

    match likes {
        Ok(likes) => Ok(HttpResponse::Ok().json(likes)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}

#[get("/dislikes")]
pub async fn show_dislikes(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let dislikes = web::block(move || {
        let conn = &mut pool.get()?;

        find_dislikes(conn)
    })
    .await?;

    match dislikes {
        Ok(dislikes) => Ok(HttpResponse::Ok().json(dislikes)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}

#[get("/comment-likes")]
pub async fn show_comment_likes(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let comment_likes = web::block(move || {
        let conn = &mut pool.get()?;

        find_comment_likes(conn)
    })
    .await?;

    match comment_likes {
        Ok(comment_likes) => Ok(HttpResponse::Ok().json(comment_likes)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}

#[get("/comment-dislikes")]
pub async fn show_comment_dislikes(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let comment_dislikes = web::block(move || {
        let conn = &mut pool.get()?;

        find_comment_dislikes(conn)
    })
    .await?;

    match comment_dislikes {
        Ok(comment_dislikes) => Ok(HttpResponse::Ok().json(comment_dislikes)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}

#[get("/reply-likes")]
pub async fn show_reply_likes(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let reply_likes = web::block(move || {
        let conn = &mut pool.get()?;

        find_reply_likes(conn)
    })
    .await?;

    match reply_likes {
        Ok(reply_likes) => Ok(HttpResponse::Ok().json(reply_likes)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}

#[get("/reply-dislikes")]
pub async fn show_reply_dislikes(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let reply_dislikes = web::block(move || {
        let conn = &mut pool.get()?;

        find_reply_dislikes(conn)
    })
    .await?;

    match reply_dislikes {
        Ok(reply_dislikes) => Ok(HttpResponse::Ok().json(reply_dislikes)),
        Err(error) => Ok(HttpResponse::NoContent().body(error.to_string())),
    }
}
