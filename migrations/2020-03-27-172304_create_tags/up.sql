-- Your SQL goes here
CREATE TABLE tags (
  id                INT UNSIGNED    NOT NULL    AUTO_INCREMENT,
  label             VARCHAR(150)    NOT NULL    UNIQUE,
  PRIMARY KEY (id)
);
