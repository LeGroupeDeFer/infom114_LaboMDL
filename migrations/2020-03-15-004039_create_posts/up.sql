-- Your SQL goes here

SET FOREIGN_KEY_CHECKS=0;

DROP TABLE IF EXISTS votes_comments;
DROP TABLE IF EXISTS votes_posts;
DROP TABLE IF EXISTS votes;
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS posts_tags;
DROP TABLE IF EXISTS posts;

CREATE TABLE posts (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    title VARCHAR(500) NOT NULL,
    content MEDIUMTEXT NOT NULL,
    post_type VARCHAR(25) NOT NULL DEFAULT "Discussion",
    authorid INT UNSIGNED NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    nb_votes INT UNSIGNED NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT fk_posts_authorid FOREIGN KEY (authorid) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT posttype_values CHECK (post_type in ("Poll", "Idea", "Info", "Decisional", "Discussion"))
);

CREATE TABLE posts_tags (
    post_id INT UNSIGNED NOT NULL,
    tag_id INT UNSIGNED NOT NULL,
    PRIMARY KEY  (post_id,tag_id),
    CONSTRAINT fk_poststags_postid FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    CONSTRAINT fk_poststags_tagid FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

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
    CONSTRAINT comments_authorid FOREIGN KEY (authorid) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT comments_postid FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    CONSTRAINT comments_parentid FOREIGN KEY (parent_id) REFERENCES comments(id) ON DELETE CASCADE
);

CREATE TABLE votes_posts (
    post_id INT UNSIGNED NOT NULL,
    vote_authorid INT UNSIGNED NOT NULL,
    voted_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    vote_value BOOLEAN NOT NULL,
    PRIMARY KEY (post_id, vote_authorid),
    CONSTRAINT votesposts_postid FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    CONSTRAINT votesposts_votesauthorid FOREIGN KEY (vote_authorid) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE votes_comments (
    comment_id INT UNSIGNED NOT NULL,
    vote_authorid INT UNSIGNED NOT NULL,
    voted_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    vote_value BOOLEAN NOT NULL,
    PRIMARY KEY (comment_id, vote_authorid),
    CONSTRAINT votescomments_commentid FOREIGN KEY (comment_id) REFERENCES comments(id) ON DELETE CASCADE,
    CONSTRAINT votescomments_voteauthorid FOREIGN KEY (vote_authorid) REFERENCES users(id) ON DELETE CASCADE
);

SET FOREIGN_KEY_CHECKS=1;