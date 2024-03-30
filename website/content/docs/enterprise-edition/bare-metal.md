+++
title = "Kubernetes on Bare Metal"
weight = 20
sort_by = "weight"
+++

## Server Node Installation
--------------
K3s provides an installation script that is a convenient way to install it as a service on systemd or openrc based systems. This script is available at https://get.k3s.io. To install K3s using this method, just run:

### 1. Install K3s

```sh
sudo curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC='server --write-kubeconfig-mode="644"' sh -
```

### 2. Update your kubeconfig

This will copy over the K3s cube config and also set the correct IP address. This is useful if you want to use K9s for example.

```sh
cp /etc/rancher/k3s/k3s.yaml ~/.kube/config && sed -i "s,127.0.0.1,$(hostname -I | awk '{print $1}'),g" ~/.kube/config
```

### 3. Check your install

```sh
kubectl get pods
# No resources found in default namespace.
```