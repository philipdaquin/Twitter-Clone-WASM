-- Your SQL goes here

CREATE TABLE IF NOT EXISTS comments (
    id SERIAL PRIMARY KEY,
    author_id SERIAL NOT NULL,
    post_id SERIAL NOT NULL,
    body VARCHAR NOT NULL,
    media VARCHAR(250) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP 
);