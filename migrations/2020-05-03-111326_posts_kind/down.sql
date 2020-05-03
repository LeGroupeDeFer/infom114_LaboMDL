-- This file should undo anything in `up.sql`
ALTER TABLE posts DROP COLUMN kind;
ALTER TABLE posts ADD post_type VARCHAR(25) NOT NULL DEFAULT 'Discussion';
ALTER TABLE posts ADD CONSTRAINT posttype_values CHECK (post_type in ("Poll", "Idea", "Info", "Decisional", "Discussion"));