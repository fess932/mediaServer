-- Your SQL goes here
create table files
(
    id   INTEGER primary key,
    name text not null,
    path text not null unique,
    type text check (type in ('file','dir'))
);
