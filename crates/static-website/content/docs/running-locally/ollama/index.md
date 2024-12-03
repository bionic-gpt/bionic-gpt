# Ollama

Ollama is an inference engine for serving models.

You'll need to install [Ollama](https://ollama.ai/) and get it running.

Once you have it running you can use the following to connect it to Bionic.

## Configuring Ollama to listen on `0.0.0.0`.

We need to get Ollama to listen on `0.0.0.0` otherwise services from within `k3s` can't connect to it.
 Run the following

```bash
sudo sed -i '/^\[Service\]/a Environment="OLLAMA_HOST=0.0.0.0"' /etc/systemd/system/ollama.service
sudo systemctl daemon-reload
sudo systemctl restart ollama.service
```

## Test Ollama

Get you host with `hostname` then curl using that host.

```sh
curl http://pop-os:11434/api/generate -d '{
  "model": "phi",
  "prompt":"Why is the sky blue?"
}'
```

## Update the model

From the models screen you'll need to change the URL from `http://llm-api` to `http://hostname`. Where host name is the name you get when your run `hostname`.