+++
title = "Postgres and PgVector"
description = "Postgres and PgVector"
weight = 20
sort_by = "weight"
+++

BionicGPT is completely stateless apart from the data we store in Postgres.

As part of installing into production you'll need a strategy for how you manage that state (i.e. backups).

We recommend using a Kubernetes Operator to manage Postgres.

## Postgres Operator

The [Postgres Operator](https://postgres-operator.readthedocs.io/en/latest/) can manage everything from data storage to scaling.

N.B. Document how to use PgVector with the Postgres Operator.

## Bionic GPT Needs Database Secrets

However or wherever you install Postgres all that bionic needs is a Kubernete Secret so it can access the database.

The secret should be called `database-urls` and should contain the URLs for 3 postgres users.

### ReadOnly User

This will be used to make backups of the database.

### Migrations User

Has full permissions and will create all the database tables as well switch on pgvector.

### Application User

For the application, has read and write access to tables but can't create or delete tables.

### Example Secret

## Testing with Kind