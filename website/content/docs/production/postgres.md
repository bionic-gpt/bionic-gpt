+++
title = "Postgres and PgVector"
description = "Installing Locally"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 20
sort_by = "weight"

[extra]
toc = true
top = false
+++

BionicGPT is completely stateless aprt from the data we store in Postgres.

As part of installing into production you'll need a strategy for how you manage that state (i.e. backups).

We recommend using a Kubernetes Operator to manage Postgres.

## Postgres Operator

The [Postgres Operator](https://postgres-operator.readthedocs.io/en/latest/) can manage everything from data storage to scaling.

N.B. Document how to use PgVector with the Postgres Operator.