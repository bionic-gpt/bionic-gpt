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

Here we assume you have BionicGPT running locally, you'll need to chnage all references from `localhost` to the domain your using for production.

## View all Models

POST https://api.openai.com/v1/completions
POST https://api.openai.com/v1/embeddings