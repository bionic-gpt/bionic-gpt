+++
title = "Using an External LLM API"
description = "Installing Locally"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 50
sort_by = "weight"

[extra]
toc = true
top = false
+++

If you're already running an LLM that supports the Open AI API in your organisation or on your local machine you may want to connect to it instead of the one we provide.

## Remove our LLM from the docker-compose.yml

Remove the following lines form `docker-compose.yml`.

```yml

  # LocalAI with pre-loaded ggml-gpt4all-j
  llm-api:
    image: ghcr.io/purton-tech/bionicgpt-model-api:latest
```

## Configure Envoy to see your LLM.

Envoy is a reverse proxy which we use to route requests between the different containers that come together to make BionicGPT work.

We're going to need to take the existing configuration and alter it to point at your LLM.

```yml
  # Handles routing between the application, barricade and the LLM API
  envoy:
    image: ghcr.io/purton-tech/bionicgpt-envoy:1.0.3
    ports:
      - "7800:7700"
      - "7801:7701"
    volumes:
      - ./envoy.yaml:/etc/envoy/envoy.yaml
```

Then we need to create an `envoy.yaml` file with the new configuration. The file is located here [envoy.yaml](https://github.com/purton-tech/bionicgpt/blob/main/.devcontainer/envoy.yaml)

To download it.

```sh
curl https://raw.githubusercontent.com/purton-tech/bionicgpt/main/.devcontainer/envoy.yaml
```

Change the following section in the `envoy.yaml` just the `address` and `port_value` entries. The address should be `host.docker.internal` and the port is whatever port your LLM is running it's OpenAI API compatibility layer.

```yml
  # The LLM API
  - name: llm-api
    connect_timeout: 10s
    type: strict_dns
    lb_policy: round_robin
    dns_lookup_family: V4_ONLY
    load_assignment:
      cluster_name: llm-api
      endpoints:
      - lb_endpoints:
        - endpoint:
            address:
              socket_address:
                address: host.docker.internal
                port_value: 5001
```


## Could we make this easier?

Ideally we'd like to make this configurable via environment variables in the `docker-compose.yml` if you can help with this please feel free to submit a PR. 