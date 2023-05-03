CREATE TABLE persons(
    id bigint(20) unsigned not null auto_increment primary key,
    name varchar(255) not null,
    mothers_name varchar(255) not null,
    fathers_name varchar(255) not null,
    cep char(8) not null,
    birth_date date not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp null default null on update current_timestamp
) engine=innodb default charset=utf8mb4 collate=utf8mb4_unicode_ci;
