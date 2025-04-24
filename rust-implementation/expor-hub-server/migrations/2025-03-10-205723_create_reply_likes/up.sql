-- Your SQL goes here
CREATE TABLE IF NOT EXISTS reply_likes (
    id          INT PRIMARY KEY AUTO_INCREMENT,
    reply_id    INT NOT NULL REFERENCES replies(reply_id),
    like_id     INT NOT NULL REFERENCES likes(like_id)
);