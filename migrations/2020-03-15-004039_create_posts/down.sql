-- This file should undo anything in `up.sql`
SET FOREIGN_KEY_CHECKS=0;

DROP TABLE IF EXISTS votes_comments;
DROP TABLE IF EXISTS votes_posts;
DROP TABLE IF EXISTS votes;
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS posts_tags;
DROP TABLE IF EXISTS posts;

SET FOREIGN_KEY_CHECKS=1;