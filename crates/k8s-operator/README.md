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

The `.kube/config` is already mapped in by `devcontainer.json`

If that one doesn't work copy `~/.kube/config` to `tmp/kubeconfig` then

```
export KUBECONFIG=/workspace/tmp/kubeconfig 
```

Then run

```sh
cargo run --bin k8s-operator -- install
```

## Testing the Operator

```sh
cargo run --bin k8s-operator -- install --no-operator --testing --grafana --hostname-url http://192.168.178.57
```

Then

```sh
cargo run --bin k8s-operator -- operator
```