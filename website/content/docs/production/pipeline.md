+++
title = "It's a Pipeline"
description = "It's a Pipeline"
weight = 5
sort_by = "weight"
+++

BionicGPT is implemented as what's called an LLM Pipeline. Basically we integrate open source components (deployed as docker containers) with a custom user interface.

## BionicGPT responsibilities

When creating an LLM pipeline you have to decide which technologies you need, which providers to use and then how to integrate those services together.

We've made those decisions and integrated them into a tried and tested pipeline.

Here's an overview of the pipeline architecture.

1. Postgres and PgVector - For storing application data such as users, roles, chat history and so on. We use the PgVector add on to store embeddings.
1. Barricade - If you're not using single sign on we use barricade for authentication.
1. Unstructured - Manages text extraction from multiple document types.
1. Envoy - Glues various services together so they are all available on the same URL.
1. BionicGTP User Interface - An axum server running the user interface.

## Pipeline Diagram

![Alt text](../architecture.svg "BionicGPT Architetcure")