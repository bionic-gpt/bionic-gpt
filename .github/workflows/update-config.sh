#!/bin/bash 
containers=("ghcr.io/bionic-gpt/bionicgpt" "ghcr.io/bionic-gpt/bionicgpt-postgres-mcp" "ghcr.io/bionic-gpt/bionicgpt-rabbitmq" "ghcr.io/bionic-gpt/bionicgpt-rag-engine" "ghcr.io/bionic-gpt/bionicgpt-db-migrations")

# Update all the version numbers in the docker-compose example
for i in "${containers[@]}"
do
    CONFIG_NAME=$(echo $i | cut -c 20-) 
    echo "Name $CONFIG_NAME"
    sed -i "0,/$CONFIG_NAME:/{s/$CONFIG_NAME:.*$/$CONFIG_NAME:$1/}" ../../infra-as-code/docker-compose.yml
    sed -i "0,/$CONFIG_NAME:/{s/$CONFIG_NAME:.*$/$CONFIG_NAME:$1/}" ../../infra-as-code/stack.yaml
done
