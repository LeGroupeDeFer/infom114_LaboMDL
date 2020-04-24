-- Your SQL goes here
CREATE TABLE posts (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    title VARCHAR(500) NOT NULL,
    content MEDIUMTEXT NOT NULL,
    post_type VARCHAR(25) NOT NULL DEFAULT "Discussion",
    author_id INT UNSIGNED NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL,
    hidden_at TIMESTAMP NULL,
    locked_at TIMESTAMP NULL,
    votes BIGINT UNSIGNED NOT NULL DEFAULT 0,
    score BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT fk_posts_authorid FOREIGN KEY (author_id) REFERENCES users(id),
    CONSTRAINT posttype_values CHECK (post_type in ("Poll", "Idea", "Info", "Decisional", "Discussion"))
);