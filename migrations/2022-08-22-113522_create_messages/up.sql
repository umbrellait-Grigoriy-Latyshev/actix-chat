-- Your SQL goes here
create table messages (
  id SERIAL PRIMARY KEY,
  actor integer,
  text TEXT NOT NULL default ''
)