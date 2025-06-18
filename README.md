# Sunny web server

|No.| Version | Description |Date |
|---|---|---|---|
|1| v0.1.0 | Initial release |2024/11/20|

## Table of contents
* [TODO](#todo)
* [Introduction](#introduction)
* [Requirements](#requirements)
* [What is this Crate?](#what-is-this-crate)
* [How to use it?](#how-to-use-it)
## TODO
- [x] Create specification
- [x] Create Directory
- [x] Create Cargo.toml
- [x] Create main.rs
- [x] Create README.md and add "Project creation date"
- [x] Determine if the edition is in the right format.
- [x] Create configs and app.toml.
- [x] Create config.rs
- [x] Create lib.rs
- [x] Loads template files into the rust binary
- [x] Add git HEAD into the binary.
- [x] Create router.rs
- [x] Create auth.db by sqlite3.
- [x] Copy static files into the project directory.
- [x] Copy template files into the project directory.
- [x] Copy congtrollers into the project directory.
- [x] The first time running this project
- [x] Create Dockerfile.
- [x] Create Makefile.
- [x] Create CAPTCHA.
- [x] Use redis to store session
- [x] Check user login
- [ ] Create view.rs
## Introduction
This is my personal web server program.I had to make it public so that I can share it with others.
I had try to make it third time.This time I will try to make it better.
## Requirements
    Rust 、HTML5、CSS3、JavaScript

## What is this Crate?
It is a simple web server.
You can use it to write your own web pages.It is written in Rust. It is based on the Tokio crate and Axum crate.
### Command line app
It has a simple command line app.It is written in Rust and very simple to use.I named it Lychee.
Yeap,It is a specification.
It is a technique supported by some model-view-controller frameworks, in which the programmer may write a specification that describes how the application database may be used. The compiler uses this specification to generate code that the application can use to create, read, update and delete database entries, effectively treating the template as a "scaffold" on which to build a more powerful application.
## Database
It uses SQLite.Because it is simple and easy to use.
I use it to store the user information.
### create database user
```shell
sqlite3 auth.db
```
```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT, --id
    username TEXT NOT NULL UNIQUE, --username
    password_hash TEXT NOT NULL    --password
);
```
Initially, I wanted to use the MD5.
But I found that it is not secure enough.Recently, MD5 has been recognized as an insufficiently secure hashing algorithm because it is vulnerable to collision attacks.
In practice, it is recommended to use a more secure hash algorithm such as argon2 or bcrypt.

I use sqlx  to connect to the database.
### create a little tool
Because there is no linux tool like md5sum.I will write a little tool to generate the hash value.
I named it sunny_bcrypt.
## Model
It uses Axum.
It is a web framework.
## View
It uses Axum.
It is a template engine.It is very simple to use.
It is a very powerful template engine.
It is written in Rust.

## How to use it?
1.Install rust.
