-- POSTS
CREATE TABLE  posts (
    id SERIAL PRIMARY KEY,
    user_id SERIAL NOT NULL,
    slug VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    body text NOT NULL,
    featured_image text NOT NULL
);








