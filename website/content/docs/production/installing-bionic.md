+++
title = "Installing Bionic"
weight = 60
sort_by = "weight"
+++

We can now use the Bionic-GPT operator to install the Deployments, Services, ConfigMaps and secrets that make Bionic-GPT work.

```sh
kubectl apply -f https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/k8s-operator/bionic.yaml
```

