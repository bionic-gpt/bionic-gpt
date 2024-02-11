+++
title = "Installation (CPU)"
weight = 20
sort_by = "weight"
+++

## Prerequisites

The easiest way to get running with BionicGPT is with our `docker-compose.yml` file. You'll need [Docker](https://docs.docker.com/engine/install/) installed on your machine.

```sh
curl -O https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/docker-compose/docker-compose.yml
```

And run

```sh
docker-compose up
```

## Install a Model

First get the a model

```sh
docker exec ollama ollama pull llama2
```

The run it.

```sh
docker exec ollama ollama pull llama2
```

Do `/bye` to exit back to the terminal.

## Run the User Interface

You can then access the front end from `http://localhost:7800` and you'll be redirected to a registration screen.

## Registration

The first user to register with **BionicGPT** will become the system administrator. The information is kept local to your machine and your data is not sent anywhere.

![Alt text](../initial-screen.png "Start Screen")
