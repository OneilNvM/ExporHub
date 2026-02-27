-- Your SQL goes here
CREATE TABLE IF NOT EXISTS images (
    image_id        SERIAL PRIMARY KEY,
    file_path       VARCHAR(255) NOT NULL UNIQUE,
    user_id         INT REFERENCES users(user_id),
    project_id      INT REFERENCES projects(project_id),
    date_uploaded   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
);