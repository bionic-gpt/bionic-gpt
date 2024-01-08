+++
title = "Using an External LLM API"
description = "Using an External LLM API"
weight = 50
sort_by = "weight"
+++

If you're already running an LLM that supports the Open AI `chat/completions` [API](https://platform.openai.com/docs/api-reference/chat) in your organisation or on your local machine you may want to connect to it instead of the one we provide.


## Add a new model

From the add new model screen configure your model.

### Name

This must correspond to the name of the model. You an get this by running curl and using localhost with the port your OpenAI API is running on.

```sh
curl http://localhost:8080/v1/models \
	-H "Content-Type: application/json"
```

### Base Url

As we are running inside a docker compose you'll need to set it to.

`http://host.docker.internal:PORT/v1`

Where PORT is the port the API is running on.

![Alt text](../add-new-model.png "Add New Model")

## If you're on Linux

you'll need to add the following to the docker compose under the `app` section

```yaml
extra_hosts:
  - "host.docker.internal:host-gateway"
```


## (Optional) Remove our LLM from the docker-compose.yml

Remove the following lines form `docker-compose.yml`.

```yml

  # LocalAI with pre-loaded ggml-gpt4all-j
  llm-api:
    image: ghcr.io/purton-tech/bionicgpt-model-api:latest
```

However, if you do this, you'll need to configure the embeddings job.

## (Optional) Configuring the embeddings job

In the `docker-compose.yml` we'll also need to configure the embeddings job to point to your external LLM API.

Add the following environment variable `EMBEDDINGS_API_ENDPOINT` and point it to the port you are running your API on.

```yml
services:
  ...
  embeddings-job:
      image: ghcr.io/purton-tech/bionicgpt-embeddings-job:1.0.3
      environment:
        EMBEDDINGS_API_ENDPOINT: http://llm-api:5001
        APP_DATABASE_URL: postgresql://bionic_application:testpassword@db:5432/postgres?sslmode=disable
```