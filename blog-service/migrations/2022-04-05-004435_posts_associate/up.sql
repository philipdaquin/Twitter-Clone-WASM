-- Your SQL goes here\


ALTER TABLE posts ADD FOREIGN KEY (author_id) REFERENCES user_comment (id) ON DELETE CASCADE;
