-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    user_id         INT(10) PRIMARY KEY,
    username        VARCHAR(50) NOT NULL,
    email           VARCHAR(128) NOT NULL,
    password        VARCHAR(255) NOT NULL,
    profile_img     VARCHAR(255),
    followers       INT(10) NOT NULL DEFAULT 0,
    date_created    DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
);