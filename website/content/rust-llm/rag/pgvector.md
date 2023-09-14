+++
title = "Choosing a Vector Database"
description = "T"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 20
sort_by = "weight"


[extra]
toc = true
top = false
section = "rust-llm"
+++

## What do we need

To work with Large Language Models we need a database that supports a feature called [similarity search](https://en.wikipedia.org/wiki/Similarity_search#:~:text=Similarity%20search%20is%20the%20most,between%20any%20pair%20of%20objects.).

This will allow us to store documents and then use those documents as part of the prompt for our LLM.

## Why pgVector?

* Although we can find databases whose specific use case is similarity search. Postgres can support this feature with an extension called [pgVector](https://github.com/pgvector/pgvector).
* It makes to re-use Postgres knowledge that may already be on the team. This includes development, deployment and operations.
* PostgreSQL supports most of the major features of SQL:2016. Out of 177 mandatory features required for full Core conformance, PostgreSQL conforms to at least 170. In addition, there is a long list of supported optional features. It might be worth noting that at the time of writing, no current version of any database management system claims full conformance to Core SQL:2016.
* Scales Vertically and Horizontally with tools such as Citus (https://www.citusdata.com/)
* Postgres supports RLS (Row Level Security) allowing you to implement authorization at the database level.
* It can support many 1000's of transaction per second running on commodity hardware.
* NoSQL support. Postgres can store and search JSON and other types of unstructured data.
* Postgres has earned a strong reputation for its proven architecture, reliability, data integrity, robust feature set, extensibility, and the dedication of the open source community behind the software to consistently deliver performant and innovative solutions.

## Test out your Postgres installation

Postgres with pgVector is pre-installed in your `devcontainer`. To try it out run the below.

```sh
> psql $DATABASE_URL

psql (14.2 (Debian 14.2-1.pgdg110+1), server 14.1 (Debian 14.1-1.pgdg110+1))
Type "help" for help.

postgres=# \dt
Did not find any relations.
postgres=# \q
```