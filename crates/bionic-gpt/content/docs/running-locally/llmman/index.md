# llmman

[llmman](https://github.com/llmmanorg/llmman) is a local model runner that serves the Ollama API (alongside OpenAI- and Anthropic-compatible ones) on port `17434`. Models are pulled as OCI artifacts or straight from Hugging Face and served by `llama.cpp`, `vllm` or `mlx-lm`.

Because llmman speaks the Ollama API, Bionic connects to it with the built in `Ollama` adapter. The only difference is the port.

## Install and start llmman

```bash
curl -fsSL https://raw.githubusercontent.com/llmmanorg/llmman/main/install.sh | sh
llmman serve
```

## Configuring llmman to listen on `0.0.0.0`.

By default llmman binds to `127.0.0.1:17434`. Services from within `k3s` or docker compose can't reach that, so start it with `LLMMAN_HOST` set:

```bash
LLMMAN_HOST=0.0.0.0 llmman serve
```

## Run a model

```sh
llmman pull gemma4
llmman run gemma4
```

## Test llmman

Run the following to see `llmman` generate some output.

```sh
curl http://localhost:17434/v1/chat/completions \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma4",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful AI agent."
            },
            {
                "role": "user",
                "content": "Hello!"
            }
        ]
    }'
```

## Add the model in Bionic

From the models screen pick the `llmman (Local)` provider. No API key is needed.

If Bionic is running inside `k3s` change the URL from `http://host.docker.internal:17434/v1` to `http://hostname:17434/v1`. Where host name is the name you get when your run `hostname`.
