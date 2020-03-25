-- Your SQL goes here
SET FOREIGN_KEY_CHECKS=0;
DROP TABLE IF EXISTS addresses;
SET FOREIGN_KEY_CHECKS=1;

CREATE TABLE addresses (
  id                INT UNSIGNED    NOT NULL    AUTO_INCREMENT,

  street            VARCHAR(150)    NOT NULL,
  number            INT UNSIGNED    NOT NULL,
  box_number        VARCHAR(16),
  city              VARCHAR(150)    NOT NULL,
  zipcode           INT UNSIGNED     NOT NULL,
  country           VARCHAR(150)    NOT NULL,

  PRIMARY KEY (id),
  CONSTRAINT unique_address UNIQUE (street, number, box_number, city, zipcode, country)
);
