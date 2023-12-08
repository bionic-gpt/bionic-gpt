+++
title = "Embeddings on GPU"
weight = 460
sort_by = "weight"
+++

We can override the `embeddings-api` entry in `docker-compose.yml`

```sh
curl -O https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/docker-compose-embed-gpu.yml
```

And run

```sh
docker-compose -f docker-compose.yml -f docker-compose-embed-gpu.yml up
```