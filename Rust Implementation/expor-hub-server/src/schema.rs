// @generated automatically by Diesel CLI.

diesel::table! {
    comment_dislikes (id) {
        id -> Integer,
        comment_id -> Integer,
        dislike_id -> Integer,
    }
}

diesel::table! {
    comment_likes (id) {
        id -> Integer,
        comment_id -> Integer,
        like_id -> Integer,
    }
}

diesel::table! {
    comments (comment_id) {
        comment_id -> Integer,
        #[max_length = 500]
        text -> Varchar,
        date -> Date,
        user_id -> Integer,
        project_id -> Integer,
        replies -> Integer,
    }
}

diesel::table! {
    dislikes (dislike_id) {
        dislike_id -> Integer,
        date_disliked -> Date,
    }
}

diesel::table! {
    favourites (favourite_id) {
        favourite_id -> Integer,
        user_id -> Integer,
        project_id -> Integer,
        date_favourited -> Datetime,
    }
}

diesel::table! {
    follows (follow_id) {
        follow_id -> Integer,
        follower -> Integer,
        following -> Integer,
        date_followed -> Datetime,
    }
}

diesel::table! {
    images (image_id) {
        image_id -> Integer,
        #[max_length = 255]
        file_name -> Varchar,
        user_id -> Nullable<Integer>,
        project_id -> Nullable<Integer>,
        date_uploaded -> Datetime,
    }
}

diesel::table! {
    likes (like_id) {
        like_id -> Integer,
        date_liked -> Date,
    }
}

diesel::table! {
    projects (project_id) {
        project_id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 2000]
        description -> Varchar,
        favourites -> Integer,
        user_id -> Integer,
        date_created -> Datetime,
        date_updated -> Nullable<Datetime>,
    }
}

diesel::table! {
    replies (reply_id) {
        reply_id -> Integer,
        #[max_length = 500]
        text -> Varchar,
        date -> Date,
        user_id -> Integer,
    }
}

diesel::table! {
    reply_dislikes (id) {
        id -> Integer,
        reply_id -> Integer,
        dislike_id -> Integer,
    }
}

diesel::table! {
    reply_likes (id) {
        id -> Integer,
        reply_id -> Integer,
        like_id -> Integer,
    }
}

diesel::table! {
    threads (thread_id) {
        thread_id -> Integer,
        comment_id -> Integer,
        reply_id -> Integer,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 4000]
        bio -> Nullable<Varchar>,
        #[max_length = 255]
        profile_img -> Nullable<Varchar>,
        followers -> Integer,
        date_created -> Datetime,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    comment_dislikes,
    comment_likes,
    comments,
    dislikes,
    favourites,
    follows,
    images,
    likes,
    projects,
    replies,
    reply_dislikes,
    reply_likes,
    threads,
    users,
);
