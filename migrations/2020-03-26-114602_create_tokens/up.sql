-- Your SQL goes here
CREATE TABLE tokens (
  id              INT UNSIGNED  NOT NULL    AUTO_INCREMENT,
  hash            VARCHAR(32)   NOT NULL,

  creation_date   TIMESTAMP     NOT NULL    DEFAULT NOW(),
  expiration_date TIMESTAMP     NULL,

  count           INT           NOT NULL    DEFAULT 1,
  consumed        BOOL          NOT NULL    DEFAULT FALSE,

  PRIMARY KEY (id),

  CONSTRAINT unique_hash        UNIQUE (hash)
);
