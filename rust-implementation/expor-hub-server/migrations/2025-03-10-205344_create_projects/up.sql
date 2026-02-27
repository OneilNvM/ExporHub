-- Your SQL goes here
CREATE TABLE IF NOT EXISTS projects (
    project_id      SERIAL PRIMARY KEY,
    name            VARCHAR(255) NOT NULL UNIQUE,
    description     TEXT NOT NULL,
    favourites      INT NOT NULL DEFAULT 0,
    user_id         INT NOT NULL REFERENCES users(user_id),
    date_created    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    date_updated    TIMESTAMP
);

CREATE INDEX projects_idx ON projects USING GIN (to_tsvector('english', name || ' ' || description));