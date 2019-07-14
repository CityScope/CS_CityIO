-- Your SQL goes here

CREATE TABLE tables (
       ID SERIAL PRIMARY KEY,
       title VARCHAR NOT NULL,
       data JSONB NOT NULL
)
