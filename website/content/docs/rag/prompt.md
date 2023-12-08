+++
title = "The Prompt and Context Size"
weight = 20
sort_by = "weight"
+++

Retrieval augmented generation places a heavy burden on the prompt. For some setups such as [Text Generation Inference](https://huggingface.co/docs/text-generation-inference/index) from hugging face they have a hard requirement on how many tokens can be sent to the model.

We do keep a track of tokens as we add context and history to the prompt that is sent to the model.

However, the inference engines may count in a different way, to stop Bionic overflowing the context you can set a trim ratio.

## Trim Ratio

When using TGI try setting the trim ration to 80% or lower.

![Alt text](../trim-ratio.png "Uploading documents")