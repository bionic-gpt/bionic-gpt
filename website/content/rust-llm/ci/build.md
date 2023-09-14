+++
title = "Build our Containers"
description = "Build our Containers"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 10
sort_by = "weight"

[extra]
toc = true
top = false
section = "rust-llm"
+++

## Introduction

The ideal output from any CI/CD pipeline is one or more Docker containers. This allows us to separate deployment from build in a clean way. Once we have our containers we are free to choose how we deploy them whether that is Kubernetes in the cloud or on or with any other deployment service that support containers.

## Earthly

Earthly uses Dockerfile syntax for creating builds. So we can leverage our existing Dockerfile knowledge. 

Create an Earthfile with the below contents.

```Dockerfile
# https://earthly.dev/
# Fast, repeatable CI/CD with an instantly familiar syntax – like Dockerfile and Makefile had a baby.

# Set the version
VERSION 0.6

# We use our devcontainer as it has all the tools we need
FROM purtontech/rust-on-nails-devcontainer:1.1.2

ARG APP_NAME=app
ARG APP_FOLDER=app
ARG IMAGE_PREFIX=rustonnails
ARG APP_EXE_NAME=axum-server

# Version of software
ARG DBMATE_VERSION=1.15.0

# Folders
ARG AXUM_FOLDER=crates/axum-server
ARG DB_FOLDER=crates/db
ARG GRPC_API_FOLDER=crates/grpc-api
ARG PIPELINE_FOLDER=crates/asset-pipeline

# This file builds the following containers
ARG APP_IMAGE_NAME=$IMAGE_PREFIX/$APP_NAME:latest
ARG MIGRATIONS_IMAGE_NAME=$IMAGE_PREFIX/$APP_NAME-migrations:latest

WORKDIR /build

USER root

# Set up for docker in docker https://github.com/earthly/earthly/issues/1225
DO github.com/earthly/lib+INSTALL_DIND

USER vscode

all:
    BUILD +migration-container
    BUILD +app-container

npm-deps:
    COPY $PIPELINE_FOLDER/package.json $PIPELINE_FOLDER/package.json
    COPY $PIPELINE_FOLDER/package-lock.json $PIPELINE_FOLDER/package-lock.json
    RUN cd $PIPELINE_FOLDER && npm install
    SAVE ARTIFACT $PIPELINE_FOLDER/node_modules

npm-build:
    FROM +npm-deps
    COPY $PIPELINE_FOLDER $PIPELINE_FOLDER
    COPY --if-exists $GRPC_API_FOLDER $GRPC_API_FOLDER
    COPY +npm-deps/node_modules $PIPELINE_FOLDER/node_modules
    RUN cd $PIPELINE_FOLDER && npm run release
    SAVE ARTIFACT $PIPELINE_FOLDER/dist

prepare-cache:
    # Copy in all our crates
    COPY --dir crates crates
    COPY Cargo.lock Cargo.toml .
    RUN cargo chef prepare --recipe-path recipe.json --bin $AXUM_FOLDER
    SAVE ARTIFACT recipe.json

build-cache:
    COPY +prepare-cache/recipe.json ./
    RUN cargo chef cook --release --target x86_64-unknown-linux-musl
    SAVE ARTIFACT target
    SAVE ARTIFACT $CARGO_HOME cargo_home
    SAVE IMAGE --cache-hint

build:
    # Copy in all our crates
    COPY --dir crates crates
    COPY --dir Cargo.lock Cargo.toml .
    COPY +build-cache/cargo_home $CARGO_HOME
    COPY +build-cache/target target
    COPY --dir +npm-build/dist $PIPELINE_FOLDER/
    # We need to run inside docker as we need postgres running for cornucopia
    ARG DATABASE_URL=postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable
    USER root
    WITH DOCKER \
        --pull postgres:alpine
        RUN docker run -d --rm --network=host -e POSTGRES_PASSWORD=testpassword postgres:alpine \
            && while ! pg_isready --host=localhost --port=5432 --username=postgres; do sleep 1; done ;\
                dbmate --migrations-dir $DB_FOLDER/migrations up \
            && cargo build --release --target x86_64-unknown-linux-musl
    END
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$APP_EXE_NAME

# This is our migrations sidecar
migration-container:
    FROM alpine
    RUN apk add --no-cache \
        curl \
        postgresql-client \
        tzdata
    RUN curl -OL https://github.com/amacneil/dbmate/releases/download/v$DBMATE_VERSION/dbmate-linux-amd64 \
        && mv ./dbmate-linux-amd64 /usr/bin/dbmate \
        && chmod +x /usr/bin/dbmate
    COPY --dir $DB_FOLDER .
    CMD dbmate up
    SAVE IMAGE --push $MIGRATIONS_IMAGE_NAME

# Our axum server
app-container:
    FROM scratch
    COPY +build/$APP_EXE_NAME axum-server
    # Place assets in a build folder as that's where statics is expecting them.
    COPY --dir +npm-build/dist /build/$PIPELINE_FOLDER/
    COPY --dir $PIPELINE_FOLDER/images /build/$PIPELINE_FOLDER/images
    ENTRYPOINT ["./axum-server"]
    SAVE IMAGE --push $APP_IMAGE_NAME
```

## Running the Build

From the command line run

```sh
earthly -P +all
```

## Validating the build

Assuming the build completes successfully we will have built two docker images

```sh
docker images | grep rustonnails
```

Your should see

```
rustonnails/app                                                  latest           db9b3436b423   6 minutes ago    10.1MB
rustonnails/app-migrations                                       latest           64064cf0fde4   11 minutes ago   24.1MB
```

One image is our Axum server, the other is an image that runs our database migrations.