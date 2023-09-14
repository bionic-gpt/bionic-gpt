+++
title = "Database Migrations"
description = "Database Migrations"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 50
sort_by = "weight"


[extra]
toc = true
top = false
section = "rust-llm"
+++

[DBMate](https://github.com/amacneil/dbmate) is a database migration tool, to keep your database schema in sync across multiple developers and your production servers. We have pre-installed it in your `devcontainer`

After that we can setup our migrations folder and the create a users migration.


## Add a Workspace

We are going to create a workspace for our application. Create a new `Cargo.toml` file in the root folder and add the following.

```toml
[workspace]
members = [
    "crates/*",
]
```

Open up the terminal in VSCode again and run the following

```
$ cargo new --vcs=none crates/db --lib
     Created binary (application) `crates/db` package
```

You should now have a folder structure like the following.

```sh
├── .devcontainer/
│   └── ...
└── crates/
│         db/
│         │  └── lib.rs
│         └── Cargo.toml
├── Cargo.toml
└── Cargo.lock
```

## Commit your code

From the `/workspace` folder

```
$ git add .
$ git commit -m"Initial Commit"
```

## Create a Migration

```
$ dbmate new items_tables
Creating migration: crates/db/migrations/20220330110026_items_tables.sql
```

Edit the SQL file that was generated for you and add the following.

```sql
-- migrate:up

CREATE EXTENSION vector;

CREATE TABLE items (id bigserial PRIMARY KEY, embedding vector(3));


-- migrate:down
DROP TABLE items;
```

## Run the Migrations

List the migrations so we can see which have run.

```
$ dbmate status
[ ] 20220330110026_user_tables.sql

Applied: 0
Pending: 1
```

Run our new migration.

```
$ dbmate up
Applying: 20220330110026_user_tables.sql
```

And check that it worked.

```
$ psql $DATABASE_URL -c 'SELECT count(*) FROM users;'
 count 
-------
      0
(1 row)
```

Your project folders should now look like this.

```sh
├── .devcontainer/
│   └── ...
└── crates/
│         db/
│         ├── migrations
│         │   └── 20220330110026_user_tables.sql
│         └── schema.sql
├── Cargo.toml
└── Cargo.lock
```