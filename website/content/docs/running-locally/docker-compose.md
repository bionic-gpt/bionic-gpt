+++
title = "Installing Locally"
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

## Prerequisites

The easiest way to get running with BionicGPT is with our `docker-compose.yml` file. You'll need [Docker](https://docs.docker.com/engine/install/) installed on your machine.

`curl -O https://raw.githubusercontent.com/purton-tech/bionicgpt/main/docker-compose.yml`

And run

`docker-compose up`

## Upgrades

When upgrading to the latest version of BionicGPT we recommend running `docker-compose down -v` to completely delete the database.
