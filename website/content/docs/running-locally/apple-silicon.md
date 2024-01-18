+++
title = "Apple Mac (Arm)"
weight = 40
sort_by = "weight"
+++

## Ollama

You'll need to install [Ollama](https://ollama.ai/) and get it running with the `llama2` model.

Once you have that running you can use the following to connect it to Bionic.

## Check Ollama is running

```sh
curl -X POST http://localhost:11434/api/generate -d '{
  "model": "llama2",
  "prompt":"Why is the sky blue?"
 }'
```

## Prerequisites

The easiest way to get running with BionicGPT is with our `docker-compose.yml` file. You'll need [Docker](https://docs.docker.com/engine/install/) installed on your machine.

```sh
curl -O https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/docker-compose/docker-compose.yml
curl -O https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/docker-compose/docker-compose-ollama.yml
```

And run

```sh
docker-compose -f docker-compose.yml -f docker-compose-ollama.yml up
```

You can then access the front end from `http://localhost:7800` and you'll be redirected to a registration screen.

## Registration

The first user to register with **BionicGPT** will become the system administrator. The information is kept local to your machine and your data is not sent anywhere.

![Alt text](../initial-screen.png "Start Screen")

## Upgrading to a later version of BionicGPT

When upgrading to the latest version of BionicGPT we recommend running 

```sh
docker-compose -f docker-compose.yml -f docker-compose-ollama.yml down -v
```

to completely delete the database.