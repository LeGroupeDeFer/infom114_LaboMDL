-- Your SQL goes here
CREATE TABLE users (
  id                INT UNSIGNED    NOT NULL    AUTO_INCREMENT,
  email             VARCHAR(150)    NOT NULL,
  password          VARCHAR(150)    NOT NULL,
  firstname         VARCHAR(150)    NOT NULL,
  lastname          VARCHAR(150)    NOT NULL,

  address           INT UNSIGNED,
  phone             VARCHAR(150),

  creation_date     TIMESTAMP       NOT NULL    DEFAULT NOW(),
  last_connection   TIMESTAMP       NOT NULL    DEFAULT NOW(),

  token             VARCHAR(36)                 DEFAULT UUID(),
  active            BOOLEAN         NOT NULL    DEFAULT FALSE,

  PRIMARY KEY (id),
  FOREIGN KEY (address) REFERENCES addresses(id),
  CONSTRAINT unique_email UNIQUE (email),
  CONSTRAINT unique_token UNIQUE (token)
);
