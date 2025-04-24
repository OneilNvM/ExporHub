-- Your SQL goes here
CREATE TABLE IF NOT EXISTS comment_dislikes (
    id          INT PRIMARY KEY AUTO_INCREMENT,
    comment_id  INT NOT NULL REFERENCES comments(comment_id),
    dislike_id  INT NOT NULL REFERENCES dislikes(dislike_id)
);