-- Your SQL goes here
SET FOREIGN_KEY_CHECKS=0;
DROP TABLE IF EXISTS roles;
SET FOREIGN_KEY_CHECKS=1;

CREATE TABLE roles (
  id                INT UNSIGNED    NOT NULL    AUTO_INCREMENT,
  name              VARCHAR(150),

  PRIMARY KEY (id),
  CONSTRAINT unique_name UNIQUE (name)
);