+++
title = "Using the API"
description = "LLM Ops"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 20
sort_by = "weight"

[extra]
toc = true
top = false
+++

Here we assume you have BionicGPT running locally, you'll need to change all references from `localhost` to the domain your using for production.

## View all Models

```sh
$ curl -H "Authorization: Bearer $BIONIC_API_KEY" http://localhost:7700/v1/models
{"object":"list","data":[{"id":"text-embedding-ada-002","object":"model"},{"id":"ggml-gpt4all-j","object":"model"}]}
```

## Completions

```sh
$ curl http://localhost:7700/v1/chat/completions   
-H "Content-Type: application/json"   
-H "Authorization: Bearer $BIONIC_API_KEY"   
-d '{
     "model": "ggml-gpt4all-j",
     "messages": [{"role": "user", "content": "Say this is a test!"}],
     "temperature": 0.7
   }'
{"object":"chat.completion","model":"ggml-gpt4all-j","choices":[{"index":0,"finish_reason":"stop","message":{"role":"assistant","content":"I'm sorry, I don't understand what you mean. Can you please provide more context or clarify your statement?"}}],"usage":{"prompt_tokens":0,"completion_tokens":0,"total_tokens":0}}
```

## Completions Streaming

Not yet available. We're working on this see



## Embeddings

```sh
$ curl http://localhost:7700/v1/chat/completions   
-H "Content-Type: application/json"   
-H "Authorization: Bearer $BIONIC_API_KEY"   
-d '{
     "model": "ggml-gpt4all-j",
     "messages": [{"role": "user", "content": "Say this is a test!"}],
     "temperature": 0.7
   }'
{"object":"chat.completion","model":"ggml-gpt4all-j","choices":[{"index":0,"finish_reason":"stop","message":{"role":"assistant","content":"I'm sorry, I don't understand what you mean. Can you please provide more context or clarify your statement?"}}],"usage":{"prompt_tokens":0,"completion_tokens":0,"total_tokens":0}}
```