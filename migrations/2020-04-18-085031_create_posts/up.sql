-- Your SQL goes here
CREATE TABLE posts (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    title VARCHAR(500) NOT NULL,
    content MEDIUMTEXT NOT NULL,
    post_type VARCHAR(25) NOT NULL DEFAULT "Discussion",
    author_id INT UNSIGNED NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    hidden_at DATETIME,
    locked_at DATETIME,
    votes INT UNSIGNED NOT NULL DEFAULT 0,
    score INT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT fk_posts_authorid FOREIGN KEY (authorid) REFERENCES users(id),
    CONSTRAINT posttype_values CHECK (post_type in ("Poll", "Idea", "Info", "Decisional", "Discussion"))
);