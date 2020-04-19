-- Your SQL goes here
CREATE TABLE votes_comments (
    comment_id INT UNSIGNED NOT NULL,
    vote_authorid INT UNSIGNED NOT NULL,
    voted_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    vote_value BOOLEAN NOT NULL,
    PRIMARY KEY (comment_id, vote_authorid),
    CONSTRAINT votescomments_commentid FOREIGN KEY (comment_id) REFERENCES comments(id),
    CONSTRAINT votescomments_voteauthorid FOREIGN KEY (vote_authorid) REFERENCES users(id)
);