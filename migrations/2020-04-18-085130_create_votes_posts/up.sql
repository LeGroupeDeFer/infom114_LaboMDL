-- Your SQL goes here
CREATE TABLE votes_posts (
    post_id INT UNSIGNED NOT NULL,
    vote_authorid INT UNSIGNED NOT NULL,
    voted_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    vote_value BOOLEAN NOT NULL,
    PRIMARY KEY (post_id, vote_authorid),
    CONSTRAINT votesposts_postid FOREIGN KEY (post_id) REFERENCES posts(id),
    CONSTRAINT votesposts_votesauthorid FOREIGN KEY (vote_authorid) REFERENCES users(id)
);