## Testing RAG

Have K3s running locally so we can expose the chunking engine.

## Setup an Embeddings Model

i.e. Open AI

## Expose Ports from K3s

Run the script in a terminal on the host (i.e. not in the devcontainer). This will open up ports so we can access the services from our devcontainer.

```sh
cat <<EOF > open-ports.sh
# Push commands in the background, when the script exits, the commands will exit too
kubectl -n bionic-gpt port-forward --address 0.0.0.0 deployment/chunking-engine 8000:8000 & \
kubectl -n bionic-gpt port-forward --address 0.0.0.0 deployment/embeddings-api 8090:80 & \

echo "Press CTRL-C to stop port forwarding and exit the script"
wait
EOF
chmod +x ./open-ports.sh
./open-ports.sh
rm ./open-ports.sh
```

You need to chnage the embeddings model to point to 8090 and also the IP address.

## Test Unstructured API

Get the host ip address i.e. `hostname -I` and export it.

```sh
export HOST_IP_ADDRESS=192.168.178.57
```

Test

```sh
curl -X 'POST' \
  "http://$HOST_IP_ADDRESS:8000/general/v0/general" \
  -H 'accept: application/json' \
  -H 'Content-Type: multipart/form-data' \
  -F 'files=@README.md' 
```

## Run the Job

```sh
CHUNKING_ENGINE=http://$HOST_IP_ADDRESS:8000 cargo run --bin rag-engine
```