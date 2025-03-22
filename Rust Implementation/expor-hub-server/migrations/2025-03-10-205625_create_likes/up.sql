-- Your SQL goes here
CREATE TABLE IF NOT EXISTS likes (
    like_id     INT PRIMARY KEY AUTO_INCREMENT,
    date_liked  DATE NOT NULL DEFAULT (CURDATE())
);