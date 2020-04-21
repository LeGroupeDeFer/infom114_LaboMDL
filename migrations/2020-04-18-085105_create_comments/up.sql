-- Your SQL goes here
CREATE TABLE comments (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    post_id INT UNSIGNED NOT NULL,
    parent_id INT UNSIGNED DEFAULT NULL,
    content MEDIUMTEXT NOT NULL, 
    author_id INT UNSIGNED NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    hidden_at DATETIME,
    votes INT UNSIGNED NOT NULL DEFAULT 0,
    score INT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT comments_authorid FOREIGN KEY (author_id) REFERENCES users(id),
    CONSTRAINT comments_postid FOREIGN KEY (post_id) REFERENCES posts(id),
    CONSTRAINT comments_parentid FOREIGN KEY (parent_id) REFERENCES comments(id)
);