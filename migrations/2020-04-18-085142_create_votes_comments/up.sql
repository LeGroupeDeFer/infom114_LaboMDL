-- Your SQL goes here
CREATE TABLE votes_comments (
    comment_id INT UNSIGNED NOT NULL,
    user_id INT UNSIGNED NOT NULL,
    voted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    vote_value SMALLINT NOT NULL,
    PRIMARY KEY (comment_id, user_id),
    CONSTRAINT votescomments_commentid FOREIGN KEY (comment_id) REFERENCES comments(id),
    CONSTRAINT votescomments_userid FOREIGN KEY (user_id) REFERENCES users(id)
);