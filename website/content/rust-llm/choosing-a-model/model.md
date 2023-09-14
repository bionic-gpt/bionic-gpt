+++
title = "Choosing a Model"
description = "T"
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

## Selection Criteria

During development you may be constrained with what model you can use based on the hardware you have. In production hopefully you have the resources to run larger models and therefore increase performance.

- The model should support a Restful API ideally compatible with the Open AI API.
- Development model should run on modest hardware.
- Run as a docker container
- Ideally the docker container is customized for MacOS, Windows and Linux to get the best performance out of each platform.
- Production model should fully utilize GPU resources.
- Streaming. When calling the API we would like to get the generated tokens in real time. The for our user interface the user can see the results as they a re generated and not have to stare at a blank screen for tens of seconds.

## Options

- Direct with [llama.cpp server](https://github.com/ggerganov/llama.cpp/tree/master/examples/server)
- the [Local AI](https://github.com/go-skynet/LocalAI) project gives us access to models with a built in Open AI API.

## Open AI API.

There is a lot of infrastructure built around the Open AI API, for example client libraries for each programming language. We can leverage this.

The endpoints we'd ideally like to have implemented for us are the following.

- `/completions`
- `/chat/completions`
- `/embeddings`
- `/engines/<any>/embeddings`
- `/v1/completions`
- `/v1/chat/completions`
- `/v1/embeddings`

## Running locally on your development machine

We've packaged the `gpt4all` model along with local AI into a container.


## Run

To start the API

`docker run -p 8080:8080 -it --rm ghcr.io/purton-tech/bionicgpt-model-api`

and you should get

```
7:23AM DBG no galleries to load
7:23AM INF Starting LocalAI using 4 threads, with models path: /build/models
7:23AM INF LocalAI version: v1.22.0 (bed9570e48581fef474580260227a102fe8a7ff4)

 ┌───────────────────────────────────────────────────┐ 
 │                   Fiber v2.48.0                   │ 
 │               http://127.0.0.1:8080               │ 
 │       (bound on host 0.0.0.0 and port 8080)       │ 
 │                                                   │ 
 │ Handlers ............ 31  Processes ........... 1 │ 
 │ Prefork ....... Disabled  PID ................. 7 │ 
 └───────────────────────────────────────────────────┘ 
```

Then Try

`curl http://localhost:8080/v1/models`