-- Your SQL goes here
CREATE TABLE reports (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    reason MEDIUMTEXT,
    PRIMARY KEY (id)
);

CREATE TABLE posts_reports (
    post_id INT UNSIGNED NOT NULL,
    report_id INT UNSIGNED NOT NULL,
    PRIMARY KEY (post_id, report_id),
    CONSTRAINT FOREIGN KEY (post_id) REFERENCES posts(id),
    CONSTRAINT FOREIGN KEY (report_id) REFERENCES reports(id)
);

CREATE TABLE comments_reports (
    comment_id INT UNSIGNED NOT NULL,
    report_id INT UNSIGNED NOT NULL,
    PRIMARY KEY (comment_id, report_id),
    CONSTRAINT FOREIGN KEY (comment_id) REFERENCES comments(id),
    CONSTRAINT FOREIGN KEY (report_id) REFERENCES reports(id)
)