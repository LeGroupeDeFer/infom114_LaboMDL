-- Your SQL goes here
CREATE TABLE posts_tags (
    post_id INT UNSIGNED NOT NULL,
    tag_label VARCHAR(150) NOT NULL,
    PRIMARY KEY  (post_id,tag_label),
    CONSTRAINT fk_poststags_postid FOREIGN KEY (post_id) REFERENCES posts(id),
    CONSTRAINT fk_poststags_taglabel FOREIGN KEY (tag_label) REFERENCES tags(label)
);