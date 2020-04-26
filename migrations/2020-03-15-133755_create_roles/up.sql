-- Your SQL goes here
CREATE TABLE roles (
  id        INT UNSIGNED    NOT NULL AUTO_INCREMENT,
  name      VARCHAR(150)    NOT NULL,
  color     VARCHAR(10)     NOT NULL DEFAULT '#29e7cd',

  PRIMARY KEY (id),
  CONSTRAINT unique_roles_name UNIQUE (name)
);