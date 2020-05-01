-- Your SQL goes here
CREATE TABLE posts_reports (
    post_id INT UNSIGNED NOT NULL,
    user_id INT UNSIGNED NOT NULL,
    reported_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reason MEDIUMTEXT,
    PRIMARY KEY (post_id, user_id),
    CONSTRAINT FOREIGN KEY (post_id) REFERENCES posts(id),
    CONSTRAINT FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE comments_reports (
    comment_id INT UNSIGNED NOT NULL,
    user_id INT UNSIGNED NOT NULL,
    reported_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reason MEDIUMTEXT,
    PRIMARY KEY (comment_id, user_id),
    CONSTRAINT FOREIGN KEY (comment_id) REFERENCES comments(id),
    CONSTRAINT FOREIGN KEY (user_id) REFERENCES users(id)
)