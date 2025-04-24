-- Your SQL goes here
CREATE TABLE IF NOT EXISTS reply_dislikes (
    id          INT PRIMARY KEY AUTO_INCREMENT,
    reply_id    INT NOT NULL REFERENCES replies(reply_id),
    dislike_id  INT NOT NULL REFERENCES dislikes(dislike_id)
);