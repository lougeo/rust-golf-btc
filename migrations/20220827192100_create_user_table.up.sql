create table users (
    id serial primary key,
    email varchar(255) not null,
    first_name varchar(255) not null,
    last_name varchar(255) not null
)