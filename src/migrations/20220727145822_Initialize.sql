-- Add migration script here
CREATE TABLE users
(
    id          INTEGER NOT NULL PRIMARY KEY,
    username    TEXT    NOT NULL,
    avatar_hash TEXT,
    timezone    TEXT,
    offset      TEXT
);
