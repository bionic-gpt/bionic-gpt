+++
title = "K8s and an Inference Engine"
weight = 95
sort_by = "weight"
+++

If you have a Kubernetes cluster with a GPU then this section is for you.

## Text Generation Inference

We recommend you run [Hugging Face TGI](https://github.com/huggingface/text-generation-inference) it's what Hugging Face are using in production and it's geared up for batch processing of requests for the best performance.

## LiteLLM

Lite LLM is a proxy that sits between Bionic and TGI and converts the TGI rest API into an Open AI compatible `chat/completions` API which is responsible for handling prompt templating.

![Alt text](../../running-locally/arch.png "Architecture")