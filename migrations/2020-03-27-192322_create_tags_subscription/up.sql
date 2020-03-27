-- Your SQL goes here

CREATE TABLE tags_subscription (
  user              INT UNSIGNED    NOT NULL,
  label             VARCHAR(150)    NOT NULL,

  PRIMARY KEY (user, label),
  FOREIGN KEY (user) REFERENCES users(id),
  FOREIGN KEY (label) REFERENCES tags(label)
);

