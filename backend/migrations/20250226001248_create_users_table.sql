-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) NOT NULL,
    age INTEGER,
    comment TEXT,
    location VARCHAR(255),
    name VARCHAR(255),
    preferences TEXT
);