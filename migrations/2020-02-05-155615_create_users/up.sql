-- Your SQL goes here
CREATE TABLE users (
  id INT PRIMARY KEY,
  username VARCHAR(150) NOT NULL,
  password VARCHAR(150) NOT NULL,
  email VARCHAR(150),
  firstname VARCHAR(150),
  lastname VARCHAR(150)
)