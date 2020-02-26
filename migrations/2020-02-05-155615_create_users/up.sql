-- Your SQL goes here
CREATE TABLE users (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  email VARCHAR(150) NOT NULL,
  password VARCHAR(150) NOT NULL,
  firstname VARCHAR(150) NOT NULL,
  lastname VARCHAR(150) NOT NULL,
  street VARCHAR(150),
  number INT UNSIGNED,
  city VARCHAR(150),
  zipcode INT UNSIGNED,
  country VARCHAR(150),
  phone VARCHAR(150),
  PRIMARY KEY (id),
  CONSTRAINT unique_email UNIQUE (email)
)