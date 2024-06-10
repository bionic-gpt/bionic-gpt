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

In this article we'll focus on Resource Exhaustion as its the most likely DoS that you'll see in a Gen AI Application.

[Benchmarking LLM Inference Backends](https://www.bentoml.com/blog/benchmarking-llm-inference-backends)

![alt text](nvidia-a100-80gb.jpg "Data Residency")

### LLMs are Memory Bound

![alt text](llama3_70b_performance.png "Data Residency")

## We need to take care of 2 scenarios

### 1. High Volume of Queries

A high volume of queries to a large language model (LLM) can significantly strain its computational resources, leading to performance degradation and potential unavailability. 

This makes sense and is the same problem for any API that gets deployed into production.

### 2. Large Prompt and Response Sizes

LLMs have an additional problem that is not often seen with other API endpoints. A single call to an LLM can be large, especially in the case of Retrieval Augmented Generation where the **prompt may contain whole documents**.

![alt text](rag-arch.png "Data Residency")

This puts extra strain on the Inference Engine.

It also works both ways. Ask an LLM to write a __10,000 word essay__ and although the initial prompt is small the LLM will need to generate thousands of tokens.

So we need a way to limit query volumes and sizes and still give our users the best results.

## Prevention using a Gateway or Reverse Proxy

LLM Inference engines actually have a relatively simple API. Nearly all popular engines support the [Open AI API](https://platform.openai.com/docs/api-reference/completions) Completions endpoint.

So when we talk about prevention we're applying protection to more or less just one main endpoint. Below is an example of calling an LLM.

```sh
curl https://api.openai.com/v1/completions \
-H "Content-Type: application/json" \
-H "Authorization: Bearer YOUR_API_KEY" \
-d '{"model": "text-davinci-003", 
    "prompt": "Say this is a test", 
    "temperature": 0, 
    "max_tokens": 7}'
```

So ideally we'd like to put a Gateway or Reverse Proxy in front of our chosen inference engine so that we can somehow throttle requests.

Ask any budding interviewee who's studied for their [System Design](https://interviewing.io/guides/system-design-interview) concepots how they woukd solve this and they would reply [Token Buckets](https://en.wikipedia.org/wiki/Token_bucket).

### Token Buckets

A token bucket is a mechanism used in network traffic management to control the amount of data that can be sent over a network. It works by generating tokens at a fixed rate and storing them in a bucket. Each token represents the permission to send a certain amount of data. When data is sent, tokens are removed from the bucket. If the bucket is empty, no data can be sent until more tokens are added, effectively regulating the data flow and ensuring network stability.

![alt text](token-bucket.webp "Data Residency")

So ideally we want to expand on this concept and add not just rate limiting but **token usage limiting**.

### Proxies with Rate Limiting and LLM Awareness?

To be useful our proxy needs to manage request rate limiting, throttling based on over use of token/request response sizes and to be **user aware**. By user aware we mean the proxy should not just limit all users but just those users who are overusing resources.

![alt text](rate-limiter.jpeg "Data Residency")

#### Envoy, Kong and Other API Gateways

Most of the mainstream Gateways support Token Buckets and have mixed support for LLM specific API calls.

They all seem to lack the ability to support throttling based on specific users. So for example we would like to have added a header to our API call based on a particular user.

#### LLM-Lite

[LLM Lite](https://www.litellm.ai) is an LLM aware proxy and has support for rate limiting based on users which it calls a budget manager.

#### How we manage this in Bionic

Bionic comes with a built in proxy that is both user aware and API key aware.

This allows you to dynamically adjust the load on your inference engines in real time and give your users the best and most fair experience.