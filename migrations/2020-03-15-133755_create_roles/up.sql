-- Your SQL goes here
CREATE TABLE roles (
  id                INT UNSIGNED    NOT NULL    AUTO_INCREMENT,
  name              VARCHAR(150),

  PRIMARY KEY (id),
  CONSTRAINT unique_name UNIQUE (name)
);