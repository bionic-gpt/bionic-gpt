# Choosing a Model

The Gen AI Lab course standardizes on the [IBM Granite 4.0 Tiny Preview](https://www.ibm.com/new/announcements/ibm-granite-4-0-tiny-preview-sneak-peek) family because it pairs great latency on commodity CPUs with the tool-calling features we depend on for agentic AI.

## Why Granite 4.0 Tiny Preview?

- **CPU-friendly** – The model fits in memory on a fast laptop or a single devcontainer node, so every participant can reproduce the labs without a GPU.
- **Reliable tool use** – Granite exposes structured tool calls out of the box, which lets us demonstrate deterministic agents without bolting on brittle prompt hacks.
- **Enterprise pedigree** – IBM ships reviewable training data documentation and consistent versioning that matches how regulated customers vet foundation models.

## Typical Lab Setup

1. Download the Tiny Preview weights and accompanying tokenizer files from the IBM announcement page.
2. Register the artifact with your inference runtime (Ollama, LM Studio, or a custom Rust runner) using the same name that the exercises reference.
3. Enable tool routing in the runtime so JSON arguments are emitted directly into your Axum handlers.
4. Capture a baseline latency trace so you can compare optimizations later in the course.

## Next Steps

Once Granite is installed locally, move on to the runtime-specific docs (such as Ollama or Docker Compose) to attach the Bionic UI and start sending agentic prompts through the toolchain.
