## Hot Reload

```
sudo apt install inotify-tools
```

```sh
./hot-reload.sh
```

## To Build Hot Reload Conatiner

```sh
docker build -t ghcr.io/bionic-gpt/bionic-gpt:latest .
k3d image import ghcr.io/bionic-gpt/bionic-gpt:latest
kubectl patch deployment bionic-gpt -n bionic-gpt -p \
    "{\"spec\": {\"template\": {\"spec\": {\"containers\": [{\"name\": \"bionic-gpt\", \"image\": \"ghcr.io/bionic-gpt/bionic-gpt:latest\", \"imagePullPolicy\": \"Never\"}]}}}}"
```