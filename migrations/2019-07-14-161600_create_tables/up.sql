-- Your SQL goes here
--

CREATE TABLE tables (
       hash VARCHAR PRIMARY KEY,
       table_name VARCHAR NOT NULL,
       ts TIMESTAMPTZ NOT NULL DEFAULT now(),
       data JSONB NOT NULL
);
