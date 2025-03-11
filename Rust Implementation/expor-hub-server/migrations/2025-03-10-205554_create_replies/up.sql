-- Your SQL goes here
CREATE TABLE IF NOT EXISTS replies (
    reply_id    INT PRIMARY KEY,
    text        VARCHAR(500) NOT NULL,
    date        DATE NOT NULL DEFAULT (CURDATE()),
    user_id     INT NOT NULL REFERENCES users(user_id)
);