-- Your SQL goes here
CREATE TABLE shared_accounts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    account_id INTEGER NOT NULL,
    role VARCHAR NOT NULL,
    UNIQUE(user_id, account_id)
);
