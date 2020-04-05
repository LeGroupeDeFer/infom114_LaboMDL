-- Your SQL goes here

CREATE TABLE tags_subscription (
  user              INT UNSIGNED    NOT NULL,
  label             INT UNSIGNED    NOT NULL    AUTO_INCREMENT,

  PRIMARY KEY (user, label),
  FOREIGN KEY (user) REFERENCES users(id),
  FOREIGN KEY (label) REFERENCES tags(id)
);

