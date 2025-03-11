-- Your SQL goes here
CREATE TABLE IF NOT EXISTS images (
    image_id        INT PRIMARY KEY,
    file_name       VARCHAR(255) NOT NULL,
    user_id         INT NOT NULL REFERENCES users(user_id),
    project_id      INT NOT NULL REFERENCES projects(project_id),
    date_uploaded   DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
);