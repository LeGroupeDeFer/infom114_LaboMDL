-- Your SQL goes here
CREATE TABLE users_roles (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  user_id INT UNSIGNED NOT NULL,
  role_id INT UNSIGNED NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT unique_user_role_usersroles UNIQUE (user_id, role_id),
  FOREIGN KEY idx_fk_user_usersroles (user_id) REFERENCES users(id),
  FOREIGN KEY idx_fk_role_usersroles (role_id) REFERENCES roles(id)
);