# Choosing a Model

We're going to use the [IBM Granite 4.0 Tiny](https://www.ibm.com/new/announcements/ibm-granite-4-0-tiny-preview-sneak-peek) model because it pairs great latency on commodity CPUs with the tool-calling features we depend on for agentic AI.

Basically it's going to work on most PC's even without a GPU and still give surprisingly good performance.

![Alt text](./granite-4.png "Granite 4")

## Why Granite 4.0 Tiny?

- **CPU-friendly** – The model fits in memory on a fast laptop or a single devcontainer node, so every participant can reproduce the labs without a GPU.
- **Reliable tool use** – Granite exposes structured tool calls out of the box, which lets us demonstrate deterministic agents without bolting on brittle prompt hacks.
- **Enterprise pedigree** – IBM ships reviewable training data documentation and consistent versioning that matches how regulated customers vet foundation models.
