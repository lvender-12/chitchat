-- Add up migration script here
ALTER TABLE users RENAME COLUMN password_hash TO password;
