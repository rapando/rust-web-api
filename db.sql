create database if not exists `rust_web_api`;
use `rust_web_api`;

create table if not exists `department` (
    `id` int primary key auto_increment,
    `name` varchar(50) not null unique,
    `staff_count` int unsigned not null default '0',
    `created` datetime not null default current_timestamp,
    `modified` datetime not null default current_timestamp on update current_timestamp
) engine=InnoDB;

create table if not exists `user` (
    `id` bigint primary key auto_increment,
    `first_name` varchar(20) not null,
    `last_name` varchar(20) not null,
    `user_name` varchar(20) not null unique,
    `department_id` int,
    `salt` varchar(32) not null,
    `password` varchar(216) not null,
     `created` datetime not null default current_timestamp,
    `modified` datetime not null default current_timestamp on update current_timestamp,

    foreign key (`department_id`) references `department`(`id`)
) engine=InnoDB;