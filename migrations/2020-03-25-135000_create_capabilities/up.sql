-- Your SQL goes here
CREATE TABLE capabilities (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  name VARCHAR(150) NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT unique_capabilities_name UNIQUE (name)
);