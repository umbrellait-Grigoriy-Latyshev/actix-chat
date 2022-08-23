-- Your SQL goes here
alter table messages add column created_at timestamp default now();