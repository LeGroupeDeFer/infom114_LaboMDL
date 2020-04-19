-- Your SQL goes here
CREATE TABLE comments (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    post_id INT UNSIGNED NOT NULL,
    content MEDIUMTEXT NOT NULL, 
    authorid INT UNSIGNED NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    nb_votes INT UNSIGNED NOT NULL DEFAULT 0,
    parent_id INT UNSIGNED DEFAULT NULL,
    PRIMARY KEY (id),
    CONSTRAINT comments_authorid FOREIGN KEY (authorid) REFERENCES users(id),
    CONSTRAINT comments_postid FOREIGN KEY (post_id) REFERENCES posts(id),
    CONSTRAINT comments_parentid FOREIGN KEY (parent_id) REFERENCES comments(id)
);