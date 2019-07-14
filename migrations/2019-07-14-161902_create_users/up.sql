-- Your SQL goes here

CREATE TABLE users (
       id serial PRIMARY KEY,
       username VARCHAR NOT NULL,
       hash VARCHAR NOT NULL,
       ts TIMESTAMPTZ NOT NULL
)
