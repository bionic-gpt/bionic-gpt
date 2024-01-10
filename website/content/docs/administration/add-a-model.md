+++
title = "Adding a Remote Model"
weight = 5
sort_by = "weight"
+++

By "adding a remote model" we mean a model that's not running in your Kubernetes cluster. This could be a model running on a private cloud service or other providers.

You basically have 2 options.

1. The provider supports the Open AI `chat/completions` [API](https://platform.openai.com/docs/api-reference/chat).
1. They don't in which case we need to install Lite LLM.

## The provider supports the Open AI `chat/completions` API

Great news. Add the URL for your provider and any API keys directly into the models section of the user interface.

![Alt text](../../running-locally/bionic-setup.png "Adding Models")

## For providers that don't support the Open AI `chat/completions` API (Work in Progress)

Luckily the guys at [Lite LLM](https://litellm.ai/) have got you covered. They basically connect to any provider and create an Open AI `chat/completions` API endpoint to your model.

You'll need to add the following to the `Pulumi.yaml` file.

Note this is a work in progress if you need help getting this running the please raise an issue on our github.

```yml

    litellm-deployment:
        type: kubernetes:apps/v1:Deployment
        properties:
            metadata:
                name: litellm-deployment
                namespace: ${namespace}
            spec:
                selector:
                    matchLabels: ${litellmLabels}
                replicas: 1
                template:
                    metadata:
                        labels: ${litellmLabels}
                    spec:
                        containers:
                            - name: litellm
                              image: ${litellm-image}
                              ports:
                                - containerPort: 8000

    litellm:
        properties:
            metadata:
                name: litellm
                namespace: ${namespace}
            spec:
                ports:
                    - port: 3000
                      protocol: TCP
                      targetPort: 3000
                selector:
                    app: litellm
        type: kubernetes:core/v1:Service
```