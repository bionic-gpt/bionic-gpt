#!/bin/bash

# Function to check and install tools if not already installed
install_tools() {
    local tool=$1
    if ! command -v "$tool" &>/dev/null; then
        echo "Installing $tool..."
        if [[ $tool == "kind" ]]; then
            install_kind
        elif [[ $tool == "kubectl" ]]; then
            install_kubectl
        elif [[ $tool == "k9s" ]]; then
            install_k9s
        fi
    else
        echo "$tool Already Installed"
    fi
}

# Function to install Kind
install_kind() {
    curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.22.0/kind-linux-amd64
    chmod +x ./kind
    sudo mv ./kind /usr/local/bin/kind
}

# Function to install kubectl
install_kubectl() {
    sudo apt-get update
    sudo apt-get install -y kubectl
}

# Function to install k9s
install_k9s() {
    curl -Lo ./k9s https://github.com/derailed/k9s/releases/download/v0.24.15/k9s_Linux_x86_64.tar.gz
    tar -xvf k9s_Linux_x86_64.tar.gz
    sudo mv ./k9s /usr/local/bin/k9s
}

# Function to clean up previous cluster and create a new one
create_kind_cluster() {
    kind delete cluster --name bionic-gpt-cluster

    cat <<EOF >kind-config.yaml
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
  kubeadmConfigPatches:
  - |
    kind: InitConfiguration
    nodeRegistration:
      kubeletExtraArgs:
        node-labels: "ingress-ready=true"
  extraPortMappings:
  - containerPort: 443
    hostPort: 443
networking:
  # If we don't do this, then we can't connect on linux
  apiServerAddress: "0.0.0.0"
kubeadmConfigPatchesJSON6902:
- group: kubeadm.k8s.io
  version: v1beta3
  kind: ClusterConfiguration
  patch: |
    - op: add
      path: /apiServer/certSANs/-
      value: host.docker.internal
EOF


    kind create cluster --name bionic-gpt-cluster --config=./kind-config.yaml
    rm ./kind-config.yaml
    kind export kubeconfig --name bionic-gpt-cluster
}

# Function to update kubeconfig for Docker Desktop
update_kubeconfig() {
    sed -i 's,https://0.0.0.0,https://host.docker.internal,g' ~/.kube/config
}

# Function to apply Kubernetes configurations
apply_bionic_crd() {
    kubectl create namespace bionic-gpt
    kubectl apply -n bionic-gpt -f https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/k8s-operator/config/bionics.bionic-gpt.com.yaml
}

# Function to install Postrgres
install_postgres_operator() {
    kubectl apply -f https://raw.githubusercontent.com/cloudnative-pg/cloudnative-pg/release-1.22/releases/cnpg-1.22.1.yaml
}

# Function to check if Docker-in-Docker parameter is supplied and update kubeconfig
check_docker_in_docker() {
    if [[ "$@" =~ "--docker-in-docker" ]]; then
        update_kubeconfig
    fi
}

preload_images() {
    echo "Preloading unstructured and embeddings for a faster startup (This takes a long time)"
    kind --name bionic-gpt-cluster load docker-image downloads.unstructured.io/unstructured-io/unstructured-api:4ffd8bc
    kind --name bionic-gpt-cluster load docker-image ghcr.io/bionic-gpt/bionicgpt-embeddings-api:cpu-0.6
}

deploy_bionic_operator() {
    kubectl apply -f https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/k8s-operator/config/bionic-operator.yaml
}

deploy_bionic() {

    curl -LO https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/k8s-operator/config/bionic.yaml

    # Point to the ip address
    sed -i "s/localhost/$1/g" ./bionic.yaml
    sed -i "s/# pgadmin/pgadmin/g" ./bionic.yaml
    sed -i "s/# gpu: true/gpu: $2/g" ./bionic.yaml

    kubectl apply -f ./bionic.yaml
    rm ./bionic.yaml
}

install_ingress_operator() {
    kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml
}

# Main function
main() {

    if [[ "$@" =~ "--localhost" ]]; then
        address="localhost"
    else
        address=$(hostname -I | awk '{print $1}')
    fi

    if [[ "$@" =~ "--gpu" ]]; then
        gpu="true"
    else
        gpu="false"
    fi

    # Install tools if not installed
    install_tools "kind"
    install_tools "kubectl"
    install_tools "k9s"

    # Execute each step
    create_kind_cluster
    check_docker_in_docker "$@"
    install_postgres_operator
    install_ingress_operator
    apply_bionic_crd
    #preload_images

    if [[ "$@" =~ "--development" ]]; then
        echo "Not deploying operator use cargo run --bin k8s-operator"
    else
        deploy_bionic_operator
    fi
    deploy_bionic "$address" "$gpu"

    echo "Bionic-GPT available on https://$address"
}

# Run the script with parameters
main "$@"