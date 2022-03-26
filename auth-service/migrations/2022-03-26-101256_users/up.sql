-- Your SQL goes here

CREATE TABLE IF NOT EXISTS valid_roles (
    role VARCHAR(64) PRIMARY KEY
);

INSERT INTO valid_roles (role) VALUES
    ('ADMIN'),
    ('USER');

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    -- 
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    username VARCHAR(30) NOT NULL UNIQUE,
    location VARCHAR(180) DEFAULT NULL,
    email VARCHAR(128) UNIQUE NOT NULL UNIQUE,
    hash VARCHAR(122) NOT NULL,
    -- User Roles
    role VARCHAR(64) REFERENCES valid_roles (role) ON UPDATE CASCADE DEFAULT 'USER' NOT NULL
);

CREATE UNIQUE INDEX users_email_index ON users(email);