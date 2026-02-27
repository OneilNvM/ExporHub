-- Your SQL goes here
CREATE TABLE IF NOT EXISTS likes (
    like_id     SERIAL PRIMARY KEY,
    user_id     INT NOT NULL REFERENCES users(user_id),
    date_liked  DATE NOT NULL DEFAULT CURRENT_DATE
);