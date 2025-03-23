-- Your SQL goes here
CREATE TABLE IF NOT EXISTS projects (
    project_id      INT(10) PRIMARY KEY AUTO_INCREMENT,
    name            VARCHAR(255) NOT NULL UNIQUE,
    description     VARCHAR(2000) NOT NULL,
    favourites      INT(10) NOT NULL DEFAULT 0,
    user_id         INT(10) NOT NULL REFERENCES users(user_id),
    date_created    DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    date_updated    DATETIME(6)
);