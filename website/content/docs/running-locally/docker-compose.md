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

The you can cut and paste the `docker-compose.yml` below and run `docker-compose up`.

## Upgrades

When upgrading to the latest version of BionicGPT we recommend running `docker-compose down -v` to completely delete the database.

## docker-compose.yml

```yml
services:

  # LocalAI with pre-loaded ggml-gpt4all-j
  llm-api:
    image: ghcr.io/purton-tech/bionicgpt-model-api:latest

  # Handles parsing of multiple documents types.
  unstructured:
    image: quay.io/unstructured-io/unstructured-api:0.0.34
    ports:
      - "8000:8000"

  # Handles routing between the application, barricade and the LLM API
  envoy:
    image: ghcr.io/purton-tech/bionicgpt-envoy:1.1.2
    ports:
      - "7800:7700"

  # Postgres pre-loaded with pgVector
  db:
    image: ankane/pgvector
    environment:
      POSTGRES_PASSWORD: testpassword
      POSTGRES_USER: postgres
      POSTGRES_DB: finetuna
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5

  # Sets up our database tables
  migrations:
    image: ghcr.io/purton-tech/bionicgpt-db-migrations:1.1.2
    environment:
      DATABASE_URL: postgresql://postgres:testpassword@db:5432/postgres?sslmode=disable
    depends_on:
      db:
        condition: service_healthy

  # Barricade handles all /auth routes for user sign up and sign in.
  barricade:
    image: purtontech/barricade
    environment:
        # This secret key is used to encrypt cookies.
        SECRET_KEY: 190a5bf4b3cbb6c0991967ab1c48ab30790af876720f1835cbbf3820f4f5d949
        DATABASE_URL: postgresql://postgres:testpassword@db:5432/postgres?sslmode=disable
        FORWARD_URL: app
        FORWARD_PORT: 7703
        REDIRECT_URL: /app/post_registration
    depends_on:
      db:
        condition: service_healthy
      migrations:
        condition: service_completed_successfully
  
  # Our axum server delivering our user interface
  embeddings-job:
    image: ghcr.io/purton-tech/bionicgpt-embeddings-job:1.1.2
    environment:
      APP_DATABASE_URL: postgresql://ft_application:testpassword@db:5432/postgres?sslmode=disable
    depends_on:
      db:
        condition: service_healthy
      migrations:
        condition: service_completed_successfully
  
  # Our axum server delivering our user interface
  app:
    image: ghcr.io/purton-tech/bionicgpt:1.1.2
    environment:
      APP_DATABASE_URL: postgresql://ft_application:testpassword@db:5432/postgres?sslmode=disable
    depends_on:
      db:
        condition: service_healthy
      migrations:
        condition: service_completed_successfully
```