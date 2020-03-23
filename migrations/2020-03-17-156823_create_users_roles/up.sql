-- Your SQL goes here
CREATE TABLE users_roles (
  user              INT UNSIGNED    NOT NULL,
  role              INT UNSIGNED    NOT NULL,

  PRIMARY KEY (user, role),
  FOREIGN KEY (user) REFERENCES users(id),
  FOREIGN KEY (role) REFERENCES roles(id)
);
