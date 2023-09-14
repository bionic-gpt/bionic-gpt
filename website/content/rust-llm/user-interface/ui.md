+++
title = "User Interface"
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

## Requirements

- Screens for uploading data to pgVector
- Connects to OpenAI or Local LLM's
- View documents that have been uploaded
- Pulls back data from vector database and populates context.
- Create, Edit Prompt Templates
- Share prompts
- Teams - Create invite - Keep data local.
- Authentication /  Authorisation
- SSO

## Contenders

- https://github.com/enricoros/big-agi
- chainlit (see below)
- https://github.com/Mintplex-Labs/anything-llm
- Custom build.

## Chainlit

[Chainlit](https://docs.chainlit.io/overview)  is an open-source Python package that makes it incredibly fast to build and share LLM apps. Integrate the Chainlit API in your existing code to spawn a ChatGPT-like interface in minutes!

![Chainlit](../chainlit.png)

- Waiting for Github issue https://github.com/Chainlit/chainlit/issues/178

- Chainlit? https://docs.chainlit.io/concepts/prompt-playground