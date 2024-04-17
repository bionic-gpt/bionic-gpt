#!/bin/bash 

# Remove a leadibng v if there is one
VERSION=$(echo $1 | sed 's/v//g')
sed -i "0,/version/{s/version.*$/version: $VERSION/}" ../../crates/k8s-operator/config/bionic.yaml

sed -i "0,/version/{s/version.*$/version = \"$VERSION\"/}" ../../crates/k8s-operator/Cargo.toml

# Update all the version number of the bionic operator
sed -i "/name = \"k8s-operator\"/{n;s/.*/version = \"$VERSION\"/}" ../../Cargo.lock
sed -i "0,/bionicgpt-k8s-operator:/{s/bionicgpt-k8s-operator:.*$/bionicgpt-k8s-operator:$VERSION/}" ../../crates/k8s-operator/config/bionic-operator.yaml