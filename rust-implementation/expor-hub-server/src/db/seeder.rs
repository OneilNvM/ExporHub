use chrono::Local;
use diesel::prelude::*;
use sha2::{Digest, Sha256};

use crate::db::db_actions::connection::establish_connection;

use super::models::*;

// Make sure to run 'cargo run -- db-seed' in order to insert the data

pub fn run() -> Result<(), diesel::result::Error> {
    use crate::schema::comment_dislikes;
    use crate::schema::comment_likes;
    use crate::schema::comments;
    use crate::schema::dislikes;
    use crate::schema::favourites;
    use crate::schema::follows;
    use crate::schema::images;
    use crate::schema::likes;
    use crate::schema::projects;
    use crate::schema::replies;
    use crate::schema::reply_dislikes;
    use crate::schema::reply_likes;
    use crate::schema::threads;
    use crate::schema::users;

    let conn = &mut establish_connection();

    // --- Insert Users ---
    let mut hasher1 = Sha256::new();
    hasher1.update("hashed_password_1".as_bytes());

    let result1 = hasher1.finalize();
    let password1 = hex::encode(result1);

    let mut hasher2 = Sha256::new();
    hasher2.update("hashed_password_2".as_bytes());

    let result2 = hasher2.finalize();
    let password2 = hex::encode(result2);

    let new_users_data = vec![
        NewUser::new("john_doe", "john.doe@example.com", &password1),
        NewUser::new("jane_smith", "jane.smith@example.com", &password2),
    ];

    diesel::insert_into(users::table)
        .values(&new_users_data)
        .execute(conn)?;

    let user1 = users::table
        .filter(users::columns::username.eq("john_doe"))
        .first::<User>(conn)?;
    let user2 = users::table
        .filter(users::columns::username.eq("jane_smith"))
        .first::<User>(conn)?;

    // --- Insert Projects ---
    let new_projects_data = vec![
        NewProject::new(
            "Awesome Website",
            "A personal portfolio website built with Rust and Yew.",
            user1.user_id,
        ),
        NewProject::new(
            "Cool Mobile App",
            "A mobile application for tracking daily tasks.",
            user2.user_id,
        ),
    ];

    diesel::insert_into(projects::table)
        .values(&new_projects_data)
        .execute(conn)?;

    let project1 = projects::table
        .filter(projects::columns::name.eq("Awesome Website"))
        .first::<Project>(conn)?;
    let project2 = projects::table
        .filter(projects::columns::name.eq("Cool Mobile App"))
        .first::<Project>(conn)?;

    // --- Insert Images ---
    let new_profile_image_data = vec![NewUserImage::new("website_screenshot.png", user1.user_id)];

    let new_project_image_data = vec![NewProjectImage::new(
        "app_logo.svg",
        user2.user_id,
        project2.project_id,
    )];

    diesel::insert_into(images::table)
        .values(&new_profile_image_data)
        .execute(conn)?;

    diesel::insert_into(images::table)
        .values(&new_project_image_data)
        .execute(conn)?;

    // --- Insert Favourites ---
    let new_favourites_data = vec![NewFavourite::new(user2.user_id, project1.project_id)];

    diesel::insert_into(favourites::table)
        .values(&new_favourites_data)
        .execute(conn)?;

    // --- Insert Follows ---
    let new_follows_data = vec![NewFollow::new(user1.user_id, user2.user_id)];

    diesel::insert_into(follows::table)
        .values(&new_follows_data)
        .execute(conn)?;

    // --- Insert Comments ---
    let new_comments_data = vec![
        NewComment::new(
            "This is a great project!",
            user2.user_id,
            project1.project_id,
        ),
        NewComment::new("Looking good!", user1.user_id, project2.project_id),
    ];

    diesel::insert_into(comments::table)
        .values(&new_comments_data)
        .execute(conn)?;

    let comment1 = comments::table
        .order(comments::columns::comment_id.desc())
        .first::<Comment>(conn)?;
    let comment2 = comments::table
        .filter(comments::columns::text.eq("Looking good!"))
        .first::<Comment>(conn)?;

    // --- Insert Replies ---
    let new_replies_data = vec![NewReply::new("Thanks!", user1.user_id)];

    diesel::insert_into(replies::table)
        .values(&new_replies_data)
        .execute(conn)?;

    let reply1 = replies::table
        .order(replies::columns::reply_id.desc())
        .first::<Reply>(conn)?;

    // --- Insert Threads ---
    let new_threads_data = vec![NewThread::new(comment1.comment_id, reply1.reply_id)];

    diesel::insert_into(threads::table)
        .values(&new_threads_data)
        .execute(conn)?;

    // --- Insert Likes ---
    let new_likes_data = vec![
        NewLike {
            user_id: user1.user_id,
            date_liked: Local::now().date_naive(),
        },
        NewLike {
            user_id: user2.user_id,
            date_liked: Local::now().date_naive(),
        },
    ];

    diesel::insert_into(likes::table)
        .values(&new_likes_data)
        .execute(conn)?;

    let like1 = likes::table
        .order(likes::columns::like_id.desc())
        .first::<Like>(conn)?;
    let like2 = likes::table
        .filter(likes::columns::user_id.eq(user2.user_id))
        .first::<Like>(conn)?;

    // --- Insert Dislikes ---
    let new_dislikes_data = vec![NewDislike {
        user_id: user1.user_id,
        date_disliked: Local::now().date_naive(),
    }];

    let new_dislike = vec![NewDislike {
        user_id: user2.user_id,
        date_disliked: Local::now().date_naive(),
    }];

    diesel::insert_into(dislikes::table)
        .values(&new_dislikes_data)
        .execute(conn)?;

    let dislike1 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .first::<Dislike>(conn)?;

    diesel::insert_into(dislikes::table)
        .values(&new_dislike)
        .execute(conn)?;

    let dislike2 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .first::<Dislike>(conn)?;

    // --- Insert Comment Likes ---
    let new_comment_likes_data = vec![NewCommentLike {
        comment_id: comment1.comment_id,
        like_id: like2.like_id,
    }];

    diesel::insert_into(comment_likes::table)
        .values(&new_comment_likes_data)
        .execute(conn)?;

    // --- Insert Comment Dislikes ---
    let new_comment_dislikes_data = vec![NewCommentDislike {
        comment_id: comment2.comment_id,
        dislike_id: dislike1.dislike_id,
    }];

    diesel::insert_into(comment_dislikes::table)
        .values(&new_comment_dislikes_data)
        .execute(conn)?;

    // --- Insert Reply Likes ---
    let new_reply_likes_data = vec![NewReplyLike {
        reply_id: reply1.reply_id,
        like_id: like1.like_id,
    }];

    diesel::insert_into(reply_likes::table)
        .values(&new_reply_likes_data)
        .execute(conn)?;

    // --- Insert Reply Dislikes ---
    let new_reply_dislikes_data = vec![NewReplyDislike {
        reply_id: reply1.reply_id,
        dislike_id: dislike2.dislike_id,
    }];

    diesel::insert_into(reply_dislikes::table)
        .values(&new_reply_dislikes_data)
        .execute(conn)?;

    // Second round of inserts

    // --- Insert Users (More) ---
    let mut hasher3 = Sha256::new();
    hasher3.update("secure_password_3".as_bytes());
    let password3 = hex::encode(hasher3.finalize());

    let mut hasher4 = Sha256::new();
    hasher4.update("another_secret_4".as_bytes());
    let password4 = hex::encode(hasher4.finalize());

    let more_users_data = vec![
        NewUser::new("peter_pan", "peter.pan@neverland.com", &password3),
        NewUser::new("wendy_darling", "wendy@neverland.com", &password4),
    ];

    diesel::insert_into(users::table)
        .values(&more_users_data)
        .execute(conn)?;

    let user3 = users::table
        .filter(users::columns::username.eq("peter_pan"))
        .first::<User>(conn)?;
    let user4 = users::table
        .filter(users::columns::username.eq("wendy_darling"))
        .first::<User>(conn)?;

    // --- Insert Projects (More) ---
    let more_projects_data = vec![
        NewProject::new(
            "Lost Boys Hideout",
            "A collaborative project for the Lost Boys.",
            user3.user_id,
        ),
        NewProject::new(
            "Wendy's Stories",
            "A collection of stories told by Wendy.",
            user4.user_id,
        ),
    ];

    diesel::insert_into(projects::table)
        .values(&more_projects_data)
        .execute(conn)?;

    let project3 = projects::table
        .filter(projects::columns::name.eq("Lost Boys Hideout"))
        .first::<Project>(conn)?;
    let project4 = projects::table
        .filter(projects::columns::name.eq("Wendy's Stories"))
        .first::<Project>(conn)?;

    // --- Insert Images (More) ---
    let new_profile_image_data_2 = vec![NewUserImage::new("peter_profile.png", user3.user_id)];
    let new_profile_image_data_3 = vec![NewUserImage::new("wendy_profile.jpg", user4.user_id)];
    let new_project_image_data_2 = vec![NewProjectImage::new(
        "hideout_plan.gif",
        user3.user_id,
        project3.project_id,
    )];
    let new_project_image_data_3 = vec![NewProjectImage::new(
        "story_cover.jpeg",
        user4.user_id,
        project4.project_id,
    )];

    diesel::insert_into(images::table)
        .values(&new_profile_image_data_2)
        .execute(conn)?;

    diesel::insert_into(images::table)
        .values(&new_profile_image_data_3)
        .execute(conn)?;

    diesel::insert_into(images::table)
        .values(&new_project_image_data_2)
        .execute(conn)?;

    diesel::insert_into(images::table)
        .values(&new_project_image_data_3)
        .execute(conn)?;

    // --- Insert Favourites (More) ---
    let more_favourites_data = vec![
        NewFavourite::new(user3.user_id, project2.project_id),
        NewFavourite::new(user4.user_id, project1.project_id),
    ];

    diesel::insert_into(favourites::table)
        .values(&more_favourites_data)
        .execute(conn)?;

    // --- Insert Follows (More) ---
    let more_follows_data = vec![
        NewFollow::new(user3.user_id, user4.user_id),
        NewFollow::new(user2.user_id, user3.user_id),
    ];

    diesel::insert_into(follows::table)
        .values(&more_follows_data)
        .execute(conn)?;

    // --- Insert Comments (More) ---
    let more_comments_data = vec![
        NewComment::new("Interesting approach!", user3.user_id, project1.project_id),
        NewComment::new("Love the design!", user4.user_id, project2.project_id),
    ];

    diesel::insert_into(comments::table)
        .values(&more_comments_data)
        .execute(conn)?;

    let comment3 = comments::table
        .filter(comments::columns::text.eq("Interesting approach!"))
        .first::<Comment>(conn)?;
    let comment4 = comments::table
        .filter(comments::columns::text.eq("Love the design!"))
        .first::<Comment>(conn)?;

    // --- Insert Replies (More) ---
    let more_replies_data = vec![
        NewReply::new("Thanks for the feedback!", user2.user_id),
        NewReply::new("Glad you like it!", user1.user_id),
    ];

    diesel::insert_into(replies::table)
        .values(&more_replies_data)
        .execute(conn)?;

    let reply2 = replies::table
        .order(replies::columns::reply_id.desc())
        .first::<Reply>(conn)?;
    let reply3 = replies::table
        .filter(replies::columns::text.eq("Glad you like it!"))
        .first::<Reply>(conn)?;

    // --- Insert Threads (More) ---
    let more_threads_data = vec![
        NewThread::new(comment2.comment_id, reply2.reply_id),
        NewThread::new(comment3.comment_id, reply3.reply_id),
    ];

    diesel::insert_into(threads::table)
        .values(&more_threads_data)
        .execute(conn)?;

    // --- Insert Likes (More) ---
    let more_likes_data = vec![
        NewLike {
            user_id: user3.user_id,
            date_liked: Local::now().date_naive(),
        },
        NewLike {
            user_id: user4.user_id,
            date_liked: Local::now().date_naive(),
        },
    ];

    diesel::insert_into(likes::table)
        .values(&more_likes_data)
        .execute(conn)?;

    let like3 = likes::table
        .filter(likes::columns::user_id.eq(user3.user_id))
        .first::<Like>(conn)?;
    let like4 = likes::table
        .order(likes::columns::like_id.desc())
        .first::<Like>(conn)?;

    // --- Insert Dislikes (More) ---
    let more_dislikes_data = vec![
        NewDislike {
            user_id: user3.user_id,
            date_disliked: Local::now().date_naive(),
        },
        NewDislike {
            user_id: user4.user_id,
            date_disliked: Local::now().date_naive(),
        },
    ];

    diesel::insert_into(dislikes::table)
        .values(&more_dislikes_data)
        .execute(conn)?;

    let dislike3 = dislikes::table
        .filter(dislikes::columns::user_id.eq(user3.user_id))
        .first::<Dislike>(conn)?;
    let dislike4 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .first::<Dislike>(conn)?;

    // --- Insert Comment Likes (More) ---
    let more_comment_likes_data = vec![
        NewCommentLike {
            comment_id: comment3.comment_id,
            like_id: like1.like_id,
        },
        NewCommentLike {
            comment_id: comment4.comment_id,
            like_id: like3.like_id,
        },
    ];

    diesel::insert_into(comment_likes::table)
        .values(&more_comment_likes_data)
        .execute(conn)?;

    // --- Insert Comment Dislikes (More) ---
    let more_comment_dislikes_data = vec![
        NewCommentDislike {
            comment_id: comment1.comment_id,
            dislike_id: dislike3.dislike_id,
        },
        NewCommentDislike {
            comment_id: comment3.comment_id,
            dislike_id: dislike2.dislike_id,
        },
    ];

    diesel::insert_into(comment_dislikes::table)
        .values(&more_comment_dislikes_data)
        .execute(conn)?;

    // --- Insert Reply Likes (More) ---
    let more_reply_likes_data = vec![
        NewReplyLike {
            reply_id: reply2.reply_id,
            like_id: like4.like_id,
        },
        NewReplyLike {
            reply_id: reply3.reply_id,
            like_id: like2.like_id,
        },
    ];

    diesel::insert_into(reply_likes::table)
        .values(&more_reply_likes_data)
        .execute(conn)?;

    // --- Insert Reply Dislikes (More) ---
    let more_reply_dislikes_data = vec![
        NewReplyDislike {
            reply_id: reply2.reply_id,
            dislike_id: dislike1.dislike_id,
        },
        NewReplyDislike {
            reply_id: reply3.reply_id,
            dislike_id: dislike4.dislike_id,
        },
    ];

    diesel::insert_into(reply_dislikes::table)
        .values(&more_reply_dislikes_data)
        .execute(conn)?;
    Ok(())
}
