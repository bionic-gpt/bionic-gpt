+++
title = "Installing Locally"
description = "Installing Locally"
weight = 20
sort_by = "weight"
+++

## Prerequisites

The easiest way to get running with BionicGPT is with our `docker-compose.yml` file. You'll need [Docker](https://docs.docker.com/engine/install/) installed on your machine.

`curl -O https://raw.githubusercontent.com/purton-tech/bionicgpt/main/docker-compose.yml`

And run

`docker-compose up`

You can then access the front end from `http://localhost:7800`. You'll get a logon screen which is filled in for you.


![Alt text](../start-screen.png "Start Screen")

## Upgrades

When upgrading to the latest version of BionicGPT we recommend running `docker-compose down -v` to completely delete the database.
