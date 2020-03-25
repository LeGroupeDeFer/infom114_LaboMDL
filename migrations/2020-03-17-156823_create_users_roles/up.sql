-- Your SQL goes here
SET FOREIGN_KEY_CHECKS=0;
DROP TABLE IF EXISTS users_roles;
SET FOREIGN_KEY_CHECKS=1;

CREATE TABLE users_roles (
  user              INT UNSIGNED    NOT NULL,
  role              INT UNSIGNED    NOT NULL,

  PRIMARY KEY (user, role),
  FOREIGN KEY (user) REFERENCES users(id),
  FOREIGN KEY (role) REFERENCES roles(id)
);
