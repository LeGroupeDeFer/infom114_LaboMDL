-- Your SQL goes here
CREATE TABLE votes_posts (
    post_id INT UNSIGNED NOT NULL,
    user_id INT UNSIGNED NOT NULL,
    voted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    vote_value BOOLEAN NOT NULL,
    PRIMARY KEY (post_id, user_id),
    CONSTRAINT votesposts_postid FOREIGN KEY (post_id) REFERENCES posts(id),
    CONSTRAINT votesposts_votesuserid FOREIGN KEY (user_id) REFERENCES users(id)
);