#!/bin/bash 
sed -i "0,/version/{s/version.*$/version: $1/}" ../../crates/k8s-operator/config/bionic.yaml

sed -i "0,/version/{s/version.*$/version = \"$1\"/}" ../../crates/bionic/Cargo.toml

# Update all the version number of the bionic operator
sed -i "0,/bionicgpt-k8s-operator:/{s/bionicgpt-k8s-operator:.*$/bionicgpt-k8s-operator:$1/}" ../../crates/k8s-operator/config/bionic-operator.yaml