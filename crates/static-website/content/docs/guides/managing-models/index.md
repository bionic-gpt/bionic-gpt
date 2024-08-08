By "adding a remote model" we mean a model that's not running in your Kubernetes cluster. This could be a model running on a private cloud service or other providers.

You basically have 2 options.

1. The provider supports the Open AI `chat/completions` [API](https://platform.openai.com/docs/api-reference/chat).
1. They don't in which case we need to install Lite LLM.

## The provider supports the Open AI `chat/completions` API

Great news. Add the URL for your provider and any API keys directly into the models section of the user interface.

![Alt text](bionic-setup.png "Adding Models")

## For providers that don't support the Open AI `chat/completions` API (Work in Progress)

Luckily the guys at [Lite LLM](https://litellm.ai/) have got you covered. They basically connect to any provider and create an Open AI `chat/completions` API endpoint to your model.