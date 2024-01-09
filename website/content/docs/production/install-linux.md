+++
title = "Installing on Linux"
weight = 10
sort_by = "weight"
+++

## Installation using Kind

[Kind](https://kind.sigs.k8s.io/) is a lightweight Kubernetes you can run locally.

We install a cluster with kind and then we can install Bionic-GPT.

## Installing Kind

```sh
sudo curl -Lo /usr/local/bin/kind https://kind.sigs.k8s.io/dl/v0.17.0/kind-linux-amd64 && sudo chmod +x /usr/local/bin/kind
```

We also need `kubectl` which is 

```sh
sudo curl -L "https://storage.googleapis.com/kubernetes-release/release/`curl -s https://storage.googleapis.com/kubernetes-release/release/stable.txt`/bin/linux/amd64/kubectl" -o /usr/local/bin/kubectl && sudo chmod +x /usr/local/bin/kubectl
```

## Setting up a local cluster with Kind

**Kind** Will create a tiny Kubernetes cluster in our docker environment. We've pre-installed `kind` in our `devcontainer` so let's create a cluster.

```sh
$ kind get clusters
No kind clusters found.
```

```sh
kind create cluster --name bionic-gpt-cluster 
```

```sh
Creating cluster "bionic-gpt-cluster" ...
 ✓ Ensuring node image (kindest/node:v1.27.3) 🖼
 ✓ Preparing nodes 📦  
 ✓ Writing configuration 📜 
 ✓ Starting control-plane 🕹️ 
 ✓ Installing CNI 🔌 
 ✓ Installing StorageClass 💾 
Set kubectl context to "kind-bionic-gpt-cluster"
You can now use your cluster with:

kubectl cluster-info --context kind-bionic-gpt-cluster

Have a question, bug, or feature request? Let us know! https://kind.sigs.k8s.io/#community 🙂
```

## Interacting with our cluster

Kubernetes is administered with a command called `kubectl` let's configure `kubectl` so that it can access our cluster.

```sh
$ kind export kubeconfig --name bionic-gpt-cluster
Set kubectl context to "kind-bionic-gpt-cluster"
```

And now we can use `kubectl` to see what `pods` we have in our cluster.

```sh
$ kubectl get pods
No resources found in default namespace.
```

## Install K9's (Optional)

```sh
curl -L -s https://github.com/derailed/k9s/releases/download/v0.24.15/k9s_Linux_x86_64.tar.gz | tar xvz -C /tmp && sudo mv /tmp/k9s /usr/bin && rm -rf k9s_Linux_x86_64.tar.gz
```