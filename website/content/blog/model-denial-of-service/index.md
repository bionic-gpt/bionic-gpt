+++
title = "Model Denial of Service prevention for  production LLM applications"
date = 2024-04-10
description = "How to prevent model denial of service for production LLM applications."

[extra]
main_image = "blog/model-denial-of-service/model-denial-of-service.png"
listing_image = "blog/model-denial-of-service/model-denial-of-service.png"
author_image = "blog-authors/ian-purton.jpeg"
author = "Ian Purton"
+++

## What is Model Denial of Service?

Denial of Service (DoS) on large language models (LLMs) and other machine learning (ML) systems can disrupt normal operations or degrade performance. These attacks can take various forms, each exploiting different aspects of the model's functionality or its supporting infrastructure. [Model Denial of Service](https://genai.owasp.org/llmrisk/llm04-model-denial-of-service/) is on the Owasp Top 10 List for LLM and Generative AI applications.

Note: We call them *attacks* but **often these can be unintentional** i.e. a script left running overnight or just normal day to day usage.

Here are some of the types of DoS. 

1. Adversarial Example Attacks
**Perturbation Attacks:** Small, crafted changes to inputs can mislead the model into incorrect outputs, increasing computational load.

2. Resource Exhaustion Attacks
**Query Flooding:** Sending a high volume of queries overwhelms the model's processing capacity.

3. Algorithmic Complexity Attacks
**Input Crafting for High Complexity:** Designing inputs to exploit worst-case performance characteristics can significantly slow down the model.

4. Data Poisoning Attacks
**Injecting Malicious Data:** Introducing bad data into the training set causes long-term performance degradation.

5. Model Overload Attacks
**Concurrent Query Flooding:** Overloading the model with simultaneous queries from multiple sources exceeds its capacity for handling concurrent processes.

6. Infrastructure Attacks
**Network Saturation:** Saturating the network bandwidth of the model's server infrastructure disrupts service.

## Resource Exhaustion

In this article we'll focus on Respource Exhaustion as its the most likely DoS that you'll see in a Gen AI Application.

[Benchmarking LLM Inference Backends](https://www.bentoml.com/blog/benchmarking-llm-inference-backends)

![alt text](nvidia-a100-80gb.jpg "Data Residency")

### LLMs are Memory Bound

![alt text](llama3_70b_performance.png "Data Residency")

We need to take care of 2 scenarios

### 1. High Volume of Queries

### 2. High Token Volume

## Prevention using a Gateway or Reverse Proxy

### Token Buckets

![alt text](token-bucket.webp "Data Residency")

### Whats out there

#### LLM-Lite

#### Envoy, Kong and Other API Gateways