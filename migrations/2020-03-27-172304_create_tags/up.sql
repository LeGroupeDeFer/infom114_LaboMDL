-- Your SQL goes here
CREATE TABLE tags (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  label VARCHAR(150) NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT unique_tags_label UNIQUE (label)
);