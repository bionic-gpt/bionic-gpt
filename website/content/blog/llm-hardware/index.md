+++
title = "Hardware requirements for LLM's in production"
date = 2023-10-27
description = "What hardware is required to put LLM's into production?"

[extra]
main_image = "blog/llm-hardware/multi-gpu-llm-setup.jpg"
listing_image = "blog/llm-hardware/multi-gpu-llm-setup.jpg"
+++

The default setup for BionicGPT allows you to run a proof of concept on your laptop, but given the much publicised large resource requirements for LLM's how do we achieve this. Step forward quantisation.	

## Tokens Per Second

The number of tokens (which roughly corresponds to words) an LLM can generate a second gives us a rough idea how our LLM is performing.

Most LLM solutions stream tokens back to the user. 

The user can see the text being generated and even if the total time of generation is 20 seconds, the user can start reading straight away.

* A speed of 5 TPS in our opinion is not great but usable.
* The speed will degrade if there is more than 1 user in the queue.
* So we have to bear in mind we want TPS to be really high to give a good experience for simultaneous users.

## Tokens Per Second Per Dollar

A second metric to look at is how much are we paying for our TPS.

The idea here is to see how much vendors are marking up graphics cards that they market towards businesses compared to their gaming GPU offerings.

## TPS requirements per 1000 users

There was a thought exercise with websites that said somethig like

> The 1% rule. 1% of your user base will be logged in and 1% of those users will be active simultaneously

I'm not sure how that applies to applications running on top of LLM's.

Especially as websites generally react much faster.

So let's take a more pessimistic rule.


> The 10% rule of LLM's. 10% of your user base will be logged in and 10% of those users will be active simultaneously

That means for every 1000 users you can expect 10 users in the queue waiting for their requests to stream.

That sounds like a lot to be honest.

If you want each of those users to get a TPS of 10, then you'll need a system capable of handling 100 TPS.

## How many billion parameters do I need?

It's still a case of the more the merrier.

## Hardware Requirements by Parameters

Below we break down the hardware requirements by parameters. 

All the models are quantized to use less RAM/VRAM and quantized models give better TPS performance will only a small amount of fall off in terms of text generation.

### 7 Billion Parameters


{{ metrics(
    title1="CPU",
    data1="12",
    description1="Hello",
    title2="GPU (TPS)",
    data2="4,200",
    description2="↗︎ 400 (22%)",
    title3="GPU Commercial (TPS)",
    data3="1,200",
    description3="↘︎ 90 (14%)"
) }}

### LLama2 30B

{{ metrics(
    title1="CPU",
    data1="12",
    description1="Hello",
    title2="GPU (TPS)",
    data2="4,200",
    description2="↗︎ 400 (22%)",
    title3="GPU Commercial (TPS)",
    data3="1,200",
    description3="↘︎ 90 (14%)"
) }}

### LLama2 70B

{{ metrics(
    title1="CPU",
    data1="12",
    description1="Hello",
    title2="2 x Nvidia 4090",
    data2="38 TPS",
    description2="↗︎ 400 (22%)",
    title3="GPU Commercial (TPS)",
    data3="1,200",
    description3="↘︎ 90 (14%)"
) }}

#### Sources

* Machine Learning Compiler (MLC) 38 TPS running on 2 Nvidai 4090's

### LLama2 130B

{{ metrics(
    title1="CPU",
    data1="12",
    description1="Hello",
    title2="GPU (TPS)",
    data2="4,200",
    description2="↗︎ 400 (22%)",
    title3="GPU Commercial (TPS)",
    data3="1,200",
    description3="↘︎ 90 (14%)"
) }}

## Return of the Mac

## API Requirements

## Conclusion

## Resources
