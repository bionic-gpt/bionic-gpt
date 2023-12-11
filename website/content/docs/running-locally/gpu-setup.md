+++
title = "Installation for GPU (TGI)"
weight = 30
sort_by = "weight"
+++

### Text-Generation-Inference

[Text-Generation-Inference](https://github.com/huggingface/text-generation-inference) is a solution for deploying and serving Large Language Models (LLMs). TGI enables high-performance text generation using Tensor Parallelism and dynamic batching for the most popular open-source LLMs, including StarCoder, BLOOM, GPT-NeoX, Llama, and T5. Text Generation Inference is already used by customers such as IBM, Grammarly. The Open-Assistant initiative implements optimization for all supported model architectures.

## Configure to run with Text Generation Inference and litellm

The standard bionicGPT install comes with a CPU based llama2-7B model installed but we can access many more models using a combination of TGI and litellm and this includes using GPU for better inference timings. This setup allows you local access to the hundreds of thousands of models listed on HuggingFace. 

![Alt text](../arch.png "Architecture")


Full details of this container can be found at [HuggingFace TGI](https://github.com/huggingface/text-generation-inference)

## Installation

```sh
curl -O https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/docker-compose.yml
curl -O https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/docker-compose-gpu.yml
```

And run

```sh
docker-compose -f docker-compose.yml -f docker-compose-gpu.yml up
```

You can then access the front end from `http://localhost:7800` and you'll be redirected to a registration screen.

## Registration

The first user to register with **BionicGPT** will become the system administrator. The information is kept local to your machine and your data is not sent anywhere.

![Alt text](../initial-screen.png "Start Screen")

## Upgrading to a later version of BionicGPT

When upgrading to the latest version of BionicGPT we recommend running 

```sh
docker-compose -f docker-compose.yml -f docker-compose-gpu.yml down -v
```

to completely delete the database.
 

## Changing The Model

The `docker-compose` override files looks something like below.

```yml
services:

  tgi:
    image: ghcr.io/huggingface/text-generation-inference:1.2
    command: --model-id TheBloke/zephyr-7B-beta-AWQ --max-batch-prefill-tokens 2048 --quantize awq
    volumes:
      - ./models:/data
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]

  llm-api:
    image: ghcr.io/berriai/litellm:main-v1.10.3
    command: --model huggingface/TheBloke/zephyr-7B-beta-AWQ --api_base http://tgi/generate_stream --host 0.0.0.0 --port 3000
    platform: linux/amd64
```

To add different models from hugging face you'll need to update the `--model` in the `llm-api` section and also the `--model-id` in the `tgi` section.

You'll need to check both with [TGI](https://github.com/huggingface/text-generation-inference) and [LiteLLM](https://litellm.ai/) that you have a compatible setup.