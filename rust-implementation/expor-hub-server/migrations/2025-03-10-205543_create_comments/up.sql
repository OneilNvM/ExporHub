-- Your SQL goes here
CREATE TABLE IF NOT EXISTS comments (
    comment_id  SERIAL PRIMARY KEY,
    text        VARCHAR(500) NOT NULL,
    date        DATE NOT NULL DEFAULT CURRENT_DATE,
    user_id     INT NOT NULL REFERENCES users(user_id),
    project_id  INT NOT NULL REFERENCES projects(project_id),
    replies     INT NOT NULL DEFAULT 0
);