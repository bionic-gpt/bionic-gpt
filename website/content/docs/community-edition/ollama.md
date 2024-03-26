+++
title = "Integrating with Ollama"
weight = 29
sort_by = "weight"
+++

## Ollama

Ollama is an inference engine for serving models.

You'll need to install [Ollama](https://ollama.ai/) and get it running with the `llama2` model.

Once you have that running you can use the following to connect it to Bionic.

## Configuring Ollama

We need to get Ollama to listen on `0.0.0.0`.

Edit the systemd service by calling `sudo vi /etc/systemd/system/ollama.service`. This will open an editor.

For each environment variable, add a line Environment under section [Service]:

```service
[Service]
Environment="OLLAMA_HOST=0.0.0.0"
```

Save and exit.

Reload systemd and restart Ollama:

```sh
systemctl daemon-reload
systemctl restart ollama
```

You can run the following to view the logs

```sh
journalctl -u ollama
```

## Test Ollama

Get you host with `hostname` then curl using that host.

```sh
curl http://pop-os:11434/api/generate -d '{
  "model": "phi",
  "prompt":"Why is the sky blue?"
}'
```

## Registration

The first user to register with **BionicGPT** will become the system administrator. The information is kept local to your machine and your data is not sent anywhere.

![Alt text](../initial-screen.png "Start Screen")

## Update the model

From the models screen you'll need to change the URL from `http://llm-api` to `http://hostname`. Where host name is the name you get when your run `hostname`.