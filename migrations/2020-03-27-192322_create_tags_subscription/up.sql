-- Your SQL goes here
CREATE TABLE tags_subscription (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  user_id INT UNSIGNED NOT NULL,
  tag_id INT UNSIGNED NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT unique_user_tag_tagssubscription UNIQUE (`user_id`, tag_id),
  FOREIGN KEY (user_id) REFERENCES users(id),
  FOREIGN KEY (tag_id) REFERENCES tags(id)
);