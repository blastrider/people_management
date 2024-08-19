-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    address VARCHAR,
    phone_number VARCHAR,
    ssn VARCHAR,
    active BOOLEAN
);

