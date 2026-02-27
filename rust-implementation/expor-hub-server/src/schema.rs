// @generated automatically by Diesel CLI.

diesel::table! {
    comment_dislikes (id) {
        id -> Int4,
        comment_id -> Int4,
        dislike_id -> Int4,
    }
}

diesel::table! {
    comment_likes (id) {
        id -> Int4,
        comment_id -> Int4,
        like_id -> Int4,
    }
}

diesel::table! {
    comments (comment_id) {
        comment_id -> Int4,
        #[max_length = 500]
        text -> Varchar,
        date -> Date,
        user_id -> Int4,
        project_id -> Int4,
        replies -> Int4,
    }
}

diesel::table! {
    dislikes (dislike_id) {
        dislike_id -> Int4,
        user_id -> Int4,
        date_disliked -> Date,
    }
}

diesel::table! {
    favourites (favourite_id) {
        favourite_id -> Int4,
        user_id -> Int4,
        project_id -> Int4,
        date_favourited -> Timestamp,
    }
}

diesel::table! {
    follows (follow_id) {
        follow_id -> Int4,
        follower -> Int4,
        following -> Int4,
        date_followed -> Timestamp,
    }
}

diesel::table! {
    images (image_id) {
        image_id -> Int4,
        #[max_length = 255]
        file_path -> Varchar,
        user_id -> Nullable<Int4>,
        project_id -> Nullable<Int4>,
        date_uploaded -> Timestamp,
    }
}

diesel::table! {
    likes (like_id) {
        like_id -> Int4,
        user_id -> Int4,
        date_liked -> Date,
    }
}

diesel::table! {
    projects (project_id) {
        project_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
        favourites -> Int4,
        user_id -> Int4,
        date_created -> Timestamp,
        date_updated -> Nullable<Timestamp>,
    }
}

diesel::table! {
    replies (reply_id) {
        reply_id -> Int4,
        #[max_length = 500]
        text -> Varchar,
        date -> Date,
        user_id -> Int4,
    }
}

diesel::table! {
    reply_dislikes (id) {
        id -> Int4,
        reply_id -> Int4,
        dislike_id -> Int4,
    }
}

diesel::table! {
    reply_likes (id) {
        id -> Int4,
        reply_id -> Int4,
        like_id -> Int4,
    }
}

diesel::table! {
    threads (thread_id) {
        thread_id -> Int4,
        comment_id -> Int4,
        reply_id -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        bio -> Nullable<Text>,
        #[max_length = 255]
        profile_img -> Nullable<Varchar>,
        followers -> Int4,
        date_created -> Timestamp,
    }
}

diesel::joinable!(comment_dislikes -> comments (comment_id));
diesel::joinable!(comment_dislikes -> dislikes (dislike_id));
diesel::joinable!(comment_likes -> comments (comment_id));
diesel::joinable!(comment_likes -> likes (like_id));
diesel::joinable!(comments -> projects (project_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(dislikes -> users (user_id));
diesel::joinable!(favourites -> projects (project_id));
diesel::joinable!(favourites -> users (user_id));
diesel::joinable!(images -> projects (project_id));
diesel::joinable!(images -> users (user_id));
diesel::joinable!(likes -> users (user_id));
diesel::joinable!(projects -> users (user_id));
diesel::joinable!(replies -> users (user_id));
diesel::joinable!(reply_dislikes -> dislikes (dislike_id));
diesel::joinable!(reply_dislikes -> replies (reply_id));
diesel::joinable!(reply_likes -> likes (like_id));
diesel::joinable!(reply_likes -> replies (reply_id));
diesel::joinable!(threads -> comments (comment_id));
diesel::joinable!(threads -> replies (reply_id));

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
