+++
title = "Introduction"
description = "LLM Ops"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 10
sort_by = "weight"

[extra]
toc = true
top = false
+++

BionicGPT was designed from the beginning to run on modest hardware and then scale into more aggressive hardware when needed.

The reasons for this are twofold.

1. We wanted people to be able to test out use cases without having to worry about purchasing graphics cards.
1. We're an open source project and we want to enable contributions from the community from contributors who may not have expensive hardware.

## Managing Expectations

For installations on modest hardware we run a quantized model with 7 billion parameters. This is great as it requires only around 5GB of memory.

However typically larger models give better results. 

We were surprised sometimes at the quality of results we got back from this model. Other times results were disappointing.

We still believe this is a great way to start to look at the ways LLM's can help out in your company. We've tried to minimise the time it takes to go from idea to practical proof of concept.

## Hardware we've tested on


| OS | Architecture | Processor | Ram | Inference | Embeddings |
| --- | --- | --- | --- | --- | --- |
| PopOs (Linux) | x86 | AMD 2700x 8 Core | 16gb | Usable | Working |
| MacOs | x86 | 2.8GHz dual core i5 | 16gb | Very Slow | Working |