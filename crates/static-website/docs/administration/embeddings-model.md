+++
title = "Change Embeddings Model"
weight = 50
sort_by = "weight"
+++

You can create a `docker-compose` override file and change the model used for embeddings.

For example to change to another model from [Text Embeddings Inference](https://github.com/huggingface/text-embeddings-inference)

create a `docker-compose-embeddings.yml` file. Such as

```yml
services:
  embeddings-api:
    image: ghcr.io/huggingface/text-embeddings-inference:cpu-0.6
    platform: linux/amd64
    command: --model-id BAAI/bge-small-en-v1.5
```

And change the command entry to the model you want to use.

## Running it all

```sh
docker-compose -f docker-compose.yml -f docker-compose-embeddings.yml up
```