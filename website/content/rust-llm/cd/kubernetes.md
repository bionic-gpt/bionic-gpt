+++
title = "Deploying to Kubernetes"
description = "Deploying to Kubernetes"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 10
sort_by = "weight"


[extra]
toc = true
top = false
section = "rust-llm"
+++

## Why Kubernetes

* We can learn one way to deploy applications and re-use those skills on other projects. So rather than learning the Google, Heroku, AWS way to deploy applications we learn the Kubernetes way of deploying applications. We will be **cloud agnostic**.

* Kubernetes will handle just about every deployment scenario you can think of.

* We can test our deployments on our local machines using a tool called `kind`.

* The Kubernetes way of application deployment will be a useful skills for the next decade or more.

* For some projects we need to deploy **On Prem** and more and more companies are providing Kubernetes as a way to deploy applications in house.

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

Normally `kind` is easier to use than this but because we are in a `devcontainer` we have to use some special configuration.

```sh
kind create cluster --name nails-cluster --config=config.yaml
```

```sh$ 
kind create cluster --name nails-cluster
Creating cluster "nails-cluster" ...
 ✓ Ensuring node image (kindest/node:v1.25.3) 🖼
 ✓ Preparing nodes 📦  
 ✓ Writing configuration 📜 
 ✓ Starting control-plane 🕹️ 
 ✓ Installing CNI 🔌 
 ✓ Installing StorageClass 💾 
Set kubectl context to "kind-nails-cluster"
You can now use your cluster with:

kubectl cluster-info --context kind-nails-cluster

Have a question, bug, or feature request? Let us know! https://kind.sigs.k8s.io/#community 🙂
```

## Interacting with our cluster (Windows and MacOS)

Kubernetes is administered with a command called `kubectl` let's configure `kubectl` so that it can access our cluster.

```sh
$ kind export kubeconfig --name nails-cluster
Set kubectl context to "kind-nails-cluster"
```

We need to do a little trick so that `kubectl` can see the cluster when running inside our `devcontainer`. Run the following.

```sh
sed -i 's/0.0.0.0/host.docker.internal/g' $HOME/.kube/config
```

And now we can use `kubectl` to see what `pods` we have in our cluster.

```sh
$ kubectl get pods
No resources found in default namespace.
```

## Interacting with our cluster in Linux

Add the following to `.devcontainer/docker-compose.yml`

```yaml
    extra_hosts:
      - "host.docker.internal:host-gateway"
```

And rebuild your devcontainer

```sh
$ kind export kubeconfig --name nails-cluster
Set kubectl context to "kind-nails-cluster"
```

We need to do a little trick so that `kubectl` can see the cluster when running inside our `devcontainer`. Run the following.

```sh
sed -i 's/0.0.0.0/host.docker.internal/g' $HOME/.kube/config
```

And now we can use `kubectl` to see what `pods` we have in our cluster.

```sh
$ kubectl get pods
No resources found in default namespace.
```