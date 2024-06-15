+++
title = "Installing cloudflare"
weight = 110
sort_by = "weight"
+++

## Install Cloudflare

```sh
export TUNNEL_TOKEN=YOUR TOKEN
bionic cloudflare --tunnel $TUNNEL_TOKEN --name bionic-gpt
```

## Configuring Keycloak to stop registrations

Edit the config map.

```sh
kubectl -n bionic-gpt edit configmap keycloak
```

**registrationAllowed** Will be set to true, set it to false and save.

Restart the keycloak server.