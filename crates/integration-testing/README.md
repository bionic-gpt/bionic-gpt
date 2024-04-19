## Expose Selenium Ports

Setup all the ENV vars 

```sh
get-env
```

Expose the ports in k9s

## Run selenium

From the host

```sh
docker run -p 4444:4444 \
    -p 7900:7900 \
    --shm-size="2g" \
    -v /home/ianpurton/Documents/bionic-gpt/crates/integration-testing/files:/workspace \
    selenium/standalone-chrome:latest
```

Make sure the Postgres port is open

## Open all the ports

```sh
cat <<EOF > open-ports.sh
# Push commands in the background, when the script exits, the commands will exit too
kubectl -n bionic-gpt port-forward --address 0.0.0.0 pod/bionic-db-cluster-1 5432 & \
kubectl -n bionic-gpt port-forward --address 0.0.0.0 deployment/mailhog 8025 & \

echo "Press CTRL-C to stop port forwarding and exit the script"
wait
EOF
chmod +x ./open-ports.sh
./open-ports.sh
rm ./open-ports.sh
```