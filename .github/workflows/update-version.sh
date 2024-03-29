#!/bin/bash 
sed -i "0,/version/{s/version.*$/version: $1/}" ../../crates/k8s-operator/config/bionic.yaml

sed -i "0,/version/{s/version.*$/version = \"$1\"/}" ../../crates/k8s-operator/Cargo.toml

# Update all the version number of the bionic operator
sed -i "/name = \"k8s-operator\"/{n;s/.*/version = \"$1\"/}" ../../Cargo.lock
sed -i "0,/bionicgpt-k8s-operator:/{s/bionicgpt-k8s-operator:.*$/bionicgpt-k8s-operator:$1/}" ../../crates/k8s-operator/config/bionic-operator.yaml