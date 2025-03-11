-- Your SQL goes here
CREATE TABLE IF NOT EXISTS dislikes (
    dislike_id      INT PRIMARY KEY,
    date_disliked   DATE NOT NULL DEFAULT (CURDATE())
);