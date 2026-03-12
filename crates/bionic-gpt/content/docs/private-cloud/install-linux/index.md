# Install on a Linux VM (K3s)

This guide shows how to take a Linux VM from any provider and install [K3s](https://k3s.io/).
Once Kubernetes is ready, continue with [Installation Prerequisite](/docs/private-cloud/installation-prerequisite) to deploy Bionic.

## 1. Install K3s

```sh
sudo curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC='server --disable=traefik --write-kubeconfig-mode="644"' sh -
mkdir -p ~/.kube
cp /etc/rancher/k3s/k3s.yaml ~/.kube/config && sed -i "s,127.0.0.1,$(hostname -I | awk '{print $1}'),g" ~/.kube/config
```

## 2. Install K9s (Optional)

```sh
curl -L -s https://github.com/derailed/k9s/releases/download/v0.24.15/k9s_Linux_x86_64.tar.gz | tar xvz -C /tmp
sudo mv /tmp/k9s /usr/local/bin
rm -rf k9s_Linux_x86_64.tar.gz
```

## 3. Check your K3s install

```sh
kubectl get pods
# No resources found in default namespace.
```

## Next step

Continue with [Installation Prerequisite](/docs/private-cloud/installation-prerequisite) to deploy Bionic.

## Uninstall K3s

First we can remove K3s entirely. K3s comes with its own uninstall script.

```sh
k3s-uninstall.sh
```

Then remove k9s if you installed it.

```sh
sudo rm /usr/local/bin/k9s
```
