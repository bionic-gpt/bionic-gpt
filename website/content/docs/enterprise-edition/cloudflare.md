+++
title = "Installing Cloudflare"
weight = 110
sort_by = "weight"
+++

## Setup a Cloudflare Tunnel

From the Cloudflare dashboard go to `zero trust / networks`. Then create a tunnel and give it a name.

![Alt text](../cloudflare-tunnel.png "Cloudflare Tunnel")

- Tunnel Type - Cloudfared
- Name - bionic-gpt

## Get your cloudflare tunnel secret

This is a bit tricky as you'll need to copy paste the command line cloudflare gives you and just extract the secret.

i.e. from the command below

```sh
sudo cloudflared service install eyJhIjoiOGMyN2IyMTg1M2YwY2VhOWQ1YTFmNmUwMzAzMzUzNTIiLCJ0IjoiYWFjYz................
```

We just need

```sh
eyJhIjoiOGMyN2IyMTg1M2YwY2VhOWQ1YTFmNmUwMzAzMzUzNTIiLCJ0....................
```

## Install Cloudflare

```sh
export TUNNEL_TOKEN=eyJhIjoiOGMyN2IyMTg1M2YwY2VhOWQ1YTFmNmUwMzAzMzUzNTIiLCJ0....................
bionic cloudflare --token $TUNNEL_TOKEN --name bionic-gpt
```

## Connect to a domain

To connect to a domain you won you need to setup 2 routes as follows.

![Alt text](../cloudflare-routes.png "Cloudflare Routes")
You should now be able to access bionic via 