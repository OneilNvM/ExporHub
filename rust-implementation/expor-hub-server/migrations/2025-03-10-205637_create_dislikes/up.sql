-- Your SQL goes here
CREATE TABLE IF NOT EXISTS dislikes (
    dislike_id      SERIAL PRIMARY KEY,
    user_id         INT NOT NULL REFERENCES users(user_id),
    date_disliked   DATE NOT NULL DEFAULT CURRENT_DATE
);