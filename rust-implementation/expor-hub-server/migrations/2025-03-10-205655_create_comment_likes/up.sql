-- Your SQL goes here
CREATE TABLE IF NOT EXISTS comment_likes (
    id          INT PRIMARY KEY AUTO_INCREMENT,
    comment_id  INT NOT NULL REFERENCES comments(comment_id),
    like_id     INT NOT NULL REFERENCES likes(like_id)
);