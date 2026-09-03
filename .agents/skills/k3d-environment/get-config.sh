#!/usr/bin/env bash
set -euo pipefail

sudo apt-get update -qq
sudo apt-get install -y -qq iproute2
k3d kubeconfig write k3d-bionic --kubeconfig-merge-default

gateway=$(ip route | awk '/default/ {print $3}')
sed -i "s/127\\.0\\.0\\.1/$gateway/g; s/0\\.0\\.0\\.0/$gateway/g" "$HOME/.kube/config"
# Disable TLS verification for local dev.
sed -i '/certificate-authority-data/d' "$HOME/.kube/config"
sed -i '/cluster:/a \    insecure-skip-tls-verify: true' "$HOME/.kube/config"
echo "kubeconfig updated and TLS verification disabled"
