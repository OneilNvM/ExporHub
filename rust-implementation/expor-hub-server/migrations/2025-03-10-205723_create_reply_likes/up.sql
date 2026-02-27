-- Your SQL goes here
CREATE TABLE IF NOT EXISTS reply_likes (
    id          SERIAL PRIMARY KEY,
    reply_id    INT NOT NULL REFERENCES replies(reply_id),
    like_id     INT NOT NULL REFERENCES likes(like_id)
);