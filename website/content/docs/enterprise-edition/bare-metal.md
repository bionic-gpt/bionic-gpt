+++
title = "Kubernetes on Bare Metal"
weight = 20
sort_by = "weight"
+++

### Server Node Installation
--------------
K3s provides an installation script that is a convenient way to install it as a service on systemd or openrc based systems. This script is available at https://get.k3s.io. To install K3s using this method, just run:

#### 1. Run the installer

```sh
sudo curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC='server --write-kubeconfig-mode="644"' sh -
```

#### 2. If you want to uninstall

```sh
sudo k3s-uninstall.sh
```

### Check your install
--------------

```sh
kubectl get pods
# No resources found in default namespace.
```