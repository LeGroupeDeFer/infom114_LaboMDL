-- Your SQL goes here

DROP TABLE IF EXISTS posts;
CREATE TABLE posts (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    title VARCHAR(400),
    content MEDIUMTEXT NOT NULL, 
    authorid INT UNSIGNED NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME ON UPDATE CURRENT_TIMESTAMP,
    reply_to INT UNSIGNED, -- for replies only
    PRIMARY KEY (id),
    CONSTRAINT fk_author FOREIGN KEY (authorid) REFERENCES users(id),
    CONSTRAINT fk_parent_post FOREIGN KEY (reply_to) REFERENCES posts(id)
);

DROP TABLE IF EXISTS tags;
CREATE TABLE tags (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    description VARCHAR(100) NOT NULL,
    PRIMARY KEY  (id)
);

DROP TABLE IF EXISTS post_tag_map;
CREATE TABLE post_tag_map (
    post_id INT UNSIGNED NOT NULL,
    tag_id INT UNSIGNED NOT NULL,
    PRIMARY KEY  (post_id,tag_id),
    CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES posts(id),
    CONSTRAINT fk_tag FOREIGN KEY (tag_id) REFERENCES tags(id)
);

DROP TABLE IF EXISTS votes;
CREATE TABLE votes (
    post_id INT UNSIGNED NOT NULL,
    vote_authorid INT UNSIGNED NOT NULL,
    voted_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    is_vote_up BOOLEAN NOT NULL,
    PRIMARY KEY (post_id, vote_authorid),
    CONSTRAINT fk_vote_post FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    CONSTRAINT fk_vote_author FOREIGN KEY (vote_authorid) REFERENCES users(id) ON DELETE CASCADE
)
