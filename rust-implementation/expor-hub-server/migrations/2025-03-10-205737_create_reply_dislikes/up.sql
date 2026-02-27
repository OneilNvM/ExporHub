-- Your SQL goes here
CREATE TABLE IF NOT EXISTS reply_dislikes (
    id          SERIAL PRIMARY KEY,
    reply_id    INT NOT NULL REFERENCES replies(reply_id),
    dislike_id  INT NOT NULL REFERENCES dislikes(dislike_id)
);