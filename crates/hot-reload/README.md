## Hot Reload

We develop completely inside Kubernetes. 

To do that we need the ability to deply the web server into the cluster.

The Hot Reload container waits around for you to send it an executable, when you do it replaces 
its current executable with the new one.

## Build the Hot Reload Container

```sh
earthly -P +hot-reload
```

## Initial Deployment

K3d allows you to import images and replace existing ones.

```sh
k3d image import bionic-gpt/bionicgpt-hot-reload:latest
```

## Patch the currenly running container with hot reload

```sh
kubectl patch deployment bionic-gpt -n bionic-gpt -p \
    "{\"spec\": {\"template\": {\"spec\": {\"containers\": [{\"name\": \"bionic-gpt\", \"image\": \"bionic-gpt/bionicgpt-hot-reload:latest\", \"imagePullPolicy\": \"Never\"}]}}}}"
```

## Install inotifywait

```sh
sudo apt-get update && sudo apt-get install -y --no-install-recommends inotify-tools
```

## Push hotreload exe into hot reload

```sh
cargo build --bin hot-reload
POD_NAME=$(kubectl get pods -n bionic-gpt -l app=bionic-gpt -o jsonpath="{.items[0].metadata.name}")
kubectl cp /workspace/target/debug/hot-reload bionic-gpt/$POD_NAME:/app/new-server
kubectl cp /workspace/crates/hot-reload/new-server.txt bionic-gpt/$POD_NAME:/app/new-server.txt
```
