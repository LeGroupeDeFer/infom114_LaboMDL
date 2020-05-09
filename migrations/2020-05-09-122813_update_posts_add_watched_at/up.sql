-- Your SQL goes here
ALTER TABLE posts ADD `watched_at` TIMESTAMP NULL AFTER locked_at;