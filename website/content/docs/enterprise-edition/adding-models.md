+++
title = "Connecting to Ollama"
weight = 95
sort_by = "weight"
+++



Edit the systemd service by calling `sudo vi /etc/systemd/system/ollama.service`. This will open an editor.

For each environment variable, add a line Environment under section [Service]:

```
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

```
journalctl -u ollama
```