+++
title = "Install Alternative Models"
description = "Install Alternative Models"
weight = 45
sort_by = "weight"
+++

[LocalAI](https://localai.com) Allows you to install new models via an API which we can call with curl.

For more information about this see [https://localai.io/models/](https://localai.io/models/)

First we can expose the API to `localhost` by adding the `ports` and `environments` directive to `docker-compose.yml`

```yml
  # LocalAI with pre-loaded ggml-gpt4all-j
  llm-api:
    image: ghcr.io/purton-tech/bionicgpt-model-api:latest
    ports:
      - "8080:8080"
    environment:
      GALLERIES: '[{"name":"model-gallery", "url":"github:go-skynet/model-gallery/index.yaml"}, {"url": "github:go-skynet/model-gallery/huggingface.yaml","name":"huggingface"}]'

```

Available models are stored at the [Local AI Model Gallery](https://github.com/go-skynet/model-gallery)

## Install Llama2 7B

So for example to install `llama2 7B` we can do the following.

```sh
curl http://llm-api:8080/models/apply -H "Content-Type: application/json" -d '{
  "id": "symecloud__llama2-7b-chat-gguf__llama-2-7b-chat.gguf.q4_0.bin",
	"name": "llama2-7b-chat"
}'  
```

You'll see something like 

```sh
{"uuid":"5ab863f9-675e-11ee-a45f-0242ac120002","status":"http://localhost:8080/models/jobs/5ab863f9-675e-11ee-a45f-0242ac120002"}
```

We can then `curl`` the URL from the reply to see the progress of the download.

```sh
curl http://localhost:8080/models/jobs/5ab863f9-675e-11ee-a45f-0242ac120002
```

## Testing the new model

```sh
curl http://llm-api:8080/v1/chat/completions -H "Content-Type: application/json" -d '{
  "model": "llama2-7b-chat", 
  "messages": [{"role": "user", "content": "How are you?"}],
  "temperature": 0.1 
}'
```