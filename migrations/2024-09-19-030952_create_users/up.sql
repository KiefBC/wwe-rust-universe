-- Your SQL goes here
CREATE TABLE users
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    username   TEXT     NOT NULL UNIQUE,
    password   TEXT     NOT NULL
);