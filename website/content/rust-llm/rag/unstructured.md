+++
title = "Unstructured for document processing"
description = "Embeddings"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 70
sort_by = "weight"


[extra]
toc = true
top = false
section = "rust-llm"
+++

80% of enterprise data exists in difficult-to-use formats like HTML, PDF, CSV, PNG, PPTX, and more. 
[Unstructured](https://github.com/Unstructured-IO/unstructured) effortlessly extracts and transforms complex data for use with every major vector database and LLM framework.

![Unstructured](../unstructured.png)

## Running the Unstructured API

If you are in the `devcontainer` then unstructured is already running other wise start unstructured with docker.

Warning this image is around 3GB!!!

```sh
docker run -it -p 8000:8000 --rm quay.io/unstructured-io/unstructured-api:latest --port 8000 --host 0.0.0.0
```

## Call the API

Replace `test.pdf` with the name of a PDF file on your local machine

In the `devcontainer`.

```sh
curl -X 'POST' 'http://unstructured:8000/general/v0/general' -H 'accept: application/json' -H 'Content-Type: multipart/form-data' -F 'files=@README.md'
```

Outside the `devconatiner`

```sh
curl -X 'POST' 'http://localhost:8000/general/v0/general' -H 'accept: application/json' -H 'Content-Type: multipart/form-data' -F 'files=@test.pdf'
```