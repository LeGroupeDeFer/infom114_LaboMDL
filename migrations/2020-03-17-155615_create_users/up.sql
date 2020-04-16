-- Your SQL goes here

SET FOREIGN_KEY_CHECKS=0;
DROP TABLE IF EXISTS users;
SET FOREIGN_KEY_CHECKS=1;

CREATE TABLE users (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  email VARCHAR(150) NOT NULL,
  password VARCHAR(150) NOT NULL,
  firstname VARCHAR(150) NOT NULL,
  lastname VARCHAR(150) NOT NULL,
  address INT UNSIGNED,
  phone VARCHAR(150),
  creation_date TIMESTAMP NOT NULL DEFAULT NOW(),
  last_connection TIMESTAMP NOT NULL DEFAULT NOW(),
  token VARCHAR(36) DEFAULT UUID(),
  active BOOLEAN NOT NULL DEFAULT FALSE,
  PRIMARY KEY (id),
  FOREIGN KEY idx_fk_address_users (address) REFERENCES addresses(id),
  CONSTRAINT unique_user_email UNIQUE (email),
  CONSTRAINT unique_user_token UNIQUE (token)
);