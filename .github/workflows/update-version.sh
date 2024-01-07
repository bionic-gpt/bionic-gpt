#!/bin/bash 
sed -i "0,/version/{s/version.*$/version: $1/}" ../../crates/k8s-operator/bionic.yaml