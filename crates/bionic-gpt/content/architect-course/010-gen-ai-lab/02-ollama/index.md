# Running an Inference Engine

Ollama is an inference engine for serving models.

You'll need to install [Ollama](https://ollama.ai/) and get it running.

Once you have it running you can use the following to connect it to Bionic.

## Configuring Ollama to listen on `0.0.0.0`.

We need to get Ollama to listen on `0.0.0.0` otherwise services from within `k3s` or docker compose can't connect to it.
 Run the following

```bash
sudo sed -i '/^\[Service\]/a Environment="OLLAMA_HOST=0.0.0.0"' /etc/systemd/system/ollama.service
sudo systemctl daemon-reload
sudo systemctl restart ollama.service
```

## Run a model

```sh
ollama run granite4:tiny-h
```

## Test Ollama

Run the following to see `ollama` generate some output.

```sh
curl http://localhost:11434/v1/chat/completions \
    -H "Content-Type: application/json" \
    -d '{
        "model": "granite4:tiny-h",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant."
            },
            {
                "role": "user",
                "content": "Hello!"
            }
        ]
    }'
```