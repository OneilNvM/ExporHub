-- Your SQL goes here
CREATE TABLE IF NOT EXISTS follows (
    follow_id       SERIAL PRIMARY KEY,
    follower        INT NOT NULL REFERENCES users(user_id),
    following       INT NOT NULL REFERENCES users(user_id),
    date_followed   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
);