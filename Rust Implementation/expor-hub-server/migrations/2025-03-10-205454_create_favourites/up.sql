-- Your SQL goes here
CREATE TABLE IF NOT EXISTS favourites (
    favourite_id        INT PRIMARY KEY  AUTO_INCREMENT,
    user_id             INT NOT NULL REFERENCES users(user_id),
    project_id          INT NOT NULL REFERENCES projects(project_id),
    date_favourited     DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
);