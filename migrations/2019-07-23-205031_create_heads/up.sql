-- Your SQL goes here

CREATE TABLE heads(
       table_name VARCHAR PRIMARY KEY,
       table_hash VARCHAR REFERENCES tables(hash) NOT NULL
)
