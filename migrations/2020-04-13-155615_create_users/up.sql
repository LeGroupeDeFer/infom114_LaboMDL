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

  activation_token  INT UNSIGNED,
  recovery_token    INT UNSIGNED,
  refresh_token     INT UNSIGNED,

  active            BOOL            NOT NULL    DEFAULT FALSE,

  PRIMARY KEY (id),

  FOREIGN KEY (address)           REFERENCES addresses(id),
  FOREIGN KEY (activation_token)  REFERENCES tokens(id),
  FOREIGN KEY (recovery_token)    REFERENCES tokens(id),
  FOREIGN KEY (refresh_token)     REFERENCES tokens(id),

  CONSTRAINT unique_email         UNIQUE (email),
  CONSTRAINT unique_activation    UNIQUE (activation_token),
  CONSTRAINT unique_recovery      UNIQUE (recovery_token),
  CONSTRAINT unique_refresh       UNIQUE (refresh_token)
);
