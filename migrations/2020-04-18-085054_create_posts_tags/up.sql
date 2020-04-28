-- Your SQL goes here
CREATE TABLE posts_tags (
    post_id INT UNSIGNED NOT NULL,
    tag_id INT UNSIGNED NOT NULL,
    PRIMARY KEY  (post_id,tag_id),
    CONSTRAINT fk_poststags_postid FOREIGN KEY (post_id) REFERENCES posts(id),
    CONSTRAINT fk_poststags_tagid FOREIGN KEY (tag_id) REFERENCES tags(id)
);