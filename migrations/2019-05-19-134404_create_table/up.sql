-- Your SQL goes here
create table tables (
       id varchar primary key not null,
       ts timestamptz not null,
       name varchar not null,
       data jsonb not null
);
