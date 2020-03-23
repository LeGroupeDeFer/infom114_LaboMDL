-- Your SQL goes here
CREATE TABLE addresses (
  id                INT UNSIGNED    NOT NULL    AUTO_INCREMENT,

  street            VARCHAR(150)    NOT NULL,
  number            INT UNSIGNED    NOT NULL,
  box_number        VARCHAR(16),
  city              VARCHAR(150)    NOT NULL,
  zipcode           VARCHAR(20)     NOT NULL,
  country           VARCHAR(150)    NOT NULL,

  PRIMARY KEY (id),
  CONSTRAINT unique_address UNIQUE (street, number, box_number, city, zipcode, country)
);
