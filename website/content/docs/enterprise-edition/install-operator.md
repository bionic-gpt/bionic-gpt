+++
title = "Installing the Operator"
weight = 15
sort_by = "weight"
+++

Bionic-GPT uses a Kubernetes Operator to manage all the Deployment, Configuration, Secrets and Services that need to be installed to run a LLM application.

## Installing the CRD (Custom Resource Definition)

```sh
kubectl apply -f https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/k8s-operator/config/bionics.bionic-gpt.com.yaml
```

Run the following command to see that the operator has been applied.

```sh
$ kubectl get crds
NAME                     CREATED AT
bionics.bionic-gpt.com   2024-01-08T08:18:32Z
```

## Creating a namespace

```sh
kubectl create namespace bionic-gpt
```

## Installing the Bionic-GPT Operator

```sh
kubectl apply -n bionic-gpt -f https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/config/k8s-operator/bionic-operator.yaml
```