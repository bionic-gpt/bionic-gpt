## The Bionic Cli and Kubernetes Operator

Run and see the CLI help

```sh
cargo run --bin k8s-operator -- -h
```

## Run as an Operator

```sh
cargo run --bin k8s-operator -- operator
```

## (Re-)install K3's

```sh
# Uninstall
sudo /usr/local/bin/k3s-uninstall.sh
```

```sh
curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC='server --write-kubeconfig-mode="644"' sh -
```

## Install Bionic into a cluster

You need the kubeconfig installed. The kubeconfig will need to point to the ip address of where the cluster is installed so we break out of the devconatiner.

Then run

```sh
cargo run --bin k8s-operator -- install
```

i.e. Have some aliases to copy the config and set ip address

```sh
alias kc='kccopy && kcupdate'
alias kccopy='cp /etc/rancher/k3s/k3s.yaml LOCATION_OF_PROJECT/bionic-gpt/k3s.yaml'
alias kcupdate='address=127.0.1.1 sed -i s/127.0.0.1/192.168.178.57/g LOCATION_OF_PROJECT/bionic-gpt/k3s.yaml'
```

Then in the devcontainer run

```sh
kc
```

## Run K9s in host

```sh
k9s --kubeconfig /etc/rancher/k3s/k3s.yaml
```

