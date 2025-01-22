#!/bin/bash 
containers=("ghcr.io/bionic-gpt/bionicgpt" "ghcr.io/bionic-gpt/bionicgpt-rabbitmq" "ghcr.io/bionic-gpt/bionicgpt-rag-engine" "ghcr.io/bionic-gpt/bionicgpt-db-migrations")

# Update our custom resource
for i in "${containers[@]}"
do
    docker pull $i 
    # i.e. turn "purtontech/cloak-server" into "server"
    CONFIG_NAME=$(echo $i | cut -c 20-) 
    HASH=$(docker inspect --format='{{index .RepoDigests 0}}' $i )
    HASH=$(echo $HASH | sed 's/^.*@//' )
    echo "Name $CONFIG_NAME"
    echo "Hash $HASH"
    # Update entries i.e. hash-trace-server: sha256:c677b52b14375c03eaede5ccdf4d40ccc9b7e9ef0157fd4b0f45b0e431f0c76d
    sed -i "0,/hash-$CONFIG_NAME/{s/hash-$CONFIG_NAME.*$/hash-$CONFIG_NAME: $HASH/}" ../../crates/k8s-operator/config/bionic.yaml
done


# Update all the version numbers in the docker-compose example
for i in "${containers[@]}"
do
    CONFIG_NAME=$(echo $i | cut -c 20-) 
    echo "Name $CONFIG_NAME"
    sed -i "0,/$CONFIG_NAME:/{s/$CONFIG_NAME:.*$/$CONFIG_NAME:$1/}" ../../infra-as-code/docker-compose.yml
done