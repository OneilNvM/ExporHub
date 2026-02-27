-- Your SQL goes here
CREATE TABLE IF NOT EXISTS comment_dislikes (
    id          SERIAL PRIMARY KEY,
    comment_id  INT NOT NULL REFERENCES comments(comment_id),
    dislike_id  INT NOT NULL REFERENCES dislikes(dislike_id)
);