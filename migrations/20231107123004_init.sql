-- Your SQL goes here

drop table if exists files;
create table files
(
    name text not null,
    path text primary key,
    parent_path text not null references files,
    type text not null check (type in ('file','dir'))
);
insert into files values ('root', 'root', 'root', 'dir')
