-- Your SQL goes here
CREATE TABLE IF NOT EXISTS threads (
    thread_id       INT PRIMARY KEY,
    comment_id      INT NOT NULL REFERENCES comments(comment_id),
    reply_id        INT NOT NULL REFERENCES replies(reply_id)
);