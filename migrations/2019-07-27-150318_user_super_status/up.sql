-- Your SQL goes here
ALTER TABLE Users
ADD is_super BOOLEAN NOT NULL DEFAULT false;
