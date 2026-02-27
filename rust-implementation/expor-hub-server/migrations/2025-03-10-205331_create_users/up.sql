-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    user_id         SERIAL PRIMARY KEY,
    username        VARCHAR(50) NOT NULL UNIQUE,
    email           VARCHAR(128) NOT NULL UNIQUE,
    password        VARCHAR(255) NOT NULL,
    bio             TEXT,
    profile_img     VARCHAR(255),
    followers       INT NOT NULL DEFAULT 0,
    date_created    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
);

CREATE INDEX users_idx ON users USING GIN (to_tsvector('english', username || ' ' || bio));