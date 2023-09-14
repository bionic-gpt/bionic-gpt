## About

We are currently using https://localai.io/ to give us a Open AI compatible Rest API in various models. LocalAI already has docker images but they don't pre-load the models.

So we add the following models

- ggml-gpt4all-j
- bert (which we use for embeddings)

## Build

`docker build -t bionicgpt-model-api:latest -f Dockerfile.localai .`

## Run

To start the api

`docker run -p 8080:8080 -it --rm bionicgpt-model-api`

`curl http://localhost:8080/v1/models`

## To run from the github container repository type

`docker run -p 8080:8080 -it --rm ghcr.io/purton-tech/bionicgpt-model-api`

```sh
curl http://localhost:8080/v1/completions -H "Content-Type: application/json" -d '{
     "model": "ggml-gpt4all-j",
     "prompt": "A long time ago in a galaxy far, far away",
     "temperature": 0.7
   }'
```

Test that we can generate embeddings

```sh
curl http://localhost:8080/v1/embeddings \
  -H "Content-Type: application/json" \
  -d '{
    "input": "Your text string goes here",
    "model": "text-embedding-ada-002"
  }'
```