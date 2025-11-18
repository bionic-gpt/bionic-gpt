# The Prompt and Context Size

Retrieval augmented generation places a heavy burden on the prompt. For some setups such as [Text Generation Inference](https://huggingface.co/docs/text-generation-inference/index) from hugging face they have a hard requirement on how many tokens can be sent to the model.

We do keep a track of tokens as we add context and history to the prompt that is sent to the model.

However, the inference engines may count in a different way, to stop Bionic overflowing the context you can set a trim ratio.

## Trim Ratio

When using TGI try setting the trim ration to 80% or lower.

![Alt text](trim-ratio.png "Uploading documents")

## Max Tokens

This is how much of the available context size the LLM can use for it's reply.

We therefore take `context_size - max_tokens` as the amount of tokens we can send to the LLM.

We then use a mixture of the history of your conversation and context from your documents until we have filled our part of the available context.

It's an idea to set `max_tokens` to roughly half of `context_size`.