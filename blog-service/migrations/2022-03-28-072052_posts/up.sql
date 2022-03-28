-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS posts (
    id UUID DEFAULT UUID_generate_v4() PRIMARY KEY,
    author_id UUID NOT NULL,
    slug VARCHAR NOT NULL UNIQUE,

    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,

    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    body text NOT NULL,
    featured_image text NOT NULL
);

-- CREATE TABLE IF NOT EXISTS comments (
--     id UUID DEFAULT UUID_generate_v4() primary key,
--     author_id UUID NOT NULL,
--     post_id UUID NOT NULL,
--     body text NOT NULL,
--     created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
--     updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP, 
--     FOREIGN KEY (author_id) REFERENCES users(id),
--     FOREIGN KEY (post_id) REFERENCES posts(id)
-- );