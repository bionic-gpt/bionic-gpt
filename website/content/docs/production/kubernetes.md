+++
title = "Kubernetes"
description = "Kubernetes"
weight = 10
sort_by = "weight"
+++

We recommend Kubernetes for deploying BionicGPT.

BionicGPT is a set of docker containers the get built using Github Actions and are available in the Github container registry.

You'll need to setup a namespace in your Kubernetes cluster and also a Postgres database with the PgVector extension installed.

## Trying out a deployment locally

We recommend using [Kind](https://kind.sigs.k8s.io/) which is a version of Kubernetes you can run locally to practice deployments.

## Setting up a local cluster with Kind

**Kind** Will create a tiny Kubernetes cluster in our docker environment. We've pre-installed `kind` in our `devcontainer` so let's create a cluster.

```sh
$ kind get clusters
No kind clusters found.
```

Create a temporary file called config.yaml.

```yaml
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
networking:
  # If we don't do this, then we can't connect on linux
  apiServerAddress: "0.0.0.0"
kubeadmConfigPatchesJSON6902:
- group: kubeadm.k8s.io
  version: v1beta3
  kind: ClusterConfiguration
  patch: |
    - op: add
      path: /apiServer/certSANs/-
      value: host.docker.internal
```

Normally `kind` is easier to use than this but because we are in a `devcontainer` we have to use some special config.

```sh
kind create cluster --name bionic-gpt-cluster --config=config.yaml
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

## Interacting with our cluster (Windows and MacOs)

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