create table blog_posts (
    id serial primary key not null,
    title varchar(255) not null,
    body text not null,
    published boolean not null default false
);