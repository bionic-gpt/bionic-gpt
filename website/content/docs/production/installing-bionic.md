+++
title = "Installing Bionic"
weight = 60
sort_by = "weight"
+++

We can now use the Bionic-GPT operator to install the Deployments, Services, ConfigMaps and secrets that make Bionic-GPT work.

```sh
kubectl apply -n bionic-gpt -f https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/k8s-operator/bionic.yaml
```

## Bionic-GPT Starting up on Kubernetes

Using `k9s` move to the `bionic-gpt` namespace (Type `:` then ns).

You will see the images downloading and the progress as the containers start.

![Alt text](../bionic-startup-k9s.png "Oauth2 Proxy")

