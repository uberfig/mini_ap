-- Add up migration script here
CREATE TABLE users (
	uid 			INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name		VARCHAR(256) NOT NULL
);