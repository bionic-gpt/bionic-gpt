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