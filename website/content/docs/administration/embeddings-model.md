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
    image: ghcr.io/huggingface/text-embeddings-inference:cpu-0.2.2
    platform: linux/amd64
    command: --model-id BAAI/bge-small-en-v1.5
```

And change the command entry to the model you want to use.

## Updating the database

Currently we use `384` dimension for embeddings in the database. If you change the model and it uses different dimension you'll need to update the database column.


i.e.

```sql
ALTER TABLE chunks ALTER COLUMN embeddings TYPE vector(500);
```