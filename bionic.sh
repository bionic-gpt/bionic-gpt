#!/bin/bash

# Function to check and install tools if not already installed
install_tools() {
    local tool=$1
    if ! command -v "$tool" &>/dev/null; then
        echo "Installing $tool..."
        if [[ $tool == "kubectl" ]]; then
            install_kubectl
        elif [[ $tool == "k9s" ]]; then
            install_k9s
        fi
    else
        echo "$tool Already Installed"
    fi
}

# Function to install k9s
install_k9s() {
    curl -L -s https://github.com/derailed/k9s/releases/download/v0.24.15/k9s_Linux_x86_64.tar.gz | tar xvz -C /tmp
    sudo mv /tmp/k9s /usr/bin
    rm -rf k9s_Linux_x86_64.tar.gz
}

reset_k3s() {
    sudo /usr/local/bin/k3s-uninstall.sh
    curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC='server --write-kubeconfig-mode="644"' sh -
    mkdir -p ~/.kube
    # Copy over kubeconfig for K9s
    sudo cp /etc/rancher/k3s/k3s.yaml ~/.kube/config
    sudo chown $USER ~/.kube/config
    sed -i "s,127.0.0.1,$1,g" ~/.kube/config
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

deploy_bionic_operator() {
    kubectl apply -f https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/k8s-operator/config/bionic-operator.yaml
}

deploy_bionic() {

    curl -LO https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/k8s-operator/config/bionic.yaml

    # Point to the ip address
    sed -i "s,https://localhost,http://$1,g" ./bionic.yaml
    sed -i "s/# pgadmin/pgadmin/g" ./bionic.yaml
    sed -i "s/# gpu: true/gpu: $2/g" ./bionic.yaml
    sed -i "s/# testing: true/testing: $3/g" ./bionic.yaml

    kubectl apply -f ./bionic.yaml
    rm ./bionic.yaml
}

expose_pgadmin() {
    echo "Email and Password and Database URL"
    kubectl get secret -n bionic-gpt pgadmin -o jsonpath='{.data.email}' | base64 --decode
    echo
    kubectl get secret -n bionic-gpt pgadmin -o jsonpath='{.data.password}' | base64 --decode
    echo
    kubectl get secret -n bionic-gpt database-urls -o jsonpath='{.data.readonly-url}' | base64 --decode
    echo
}

# Main function
install() {

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

    if [[ "$@" =~ "--testing" ]]; then
        testing="true"
    else
        testing="false"
    fi

    if [[ "$@" =~ "--k3s" ]]; then
        reset_k3s "$address"
    fi

    if [[ "$@" =~ "--k9s" ]]; then
        install_tools k9s
    fi

    install_postgres_operator
    echo "Waiting for Postgres Operator to be ready"
    kubectl wait --timeout=120s --for=condition=available deployment/cnpg-controller-manager -n cnpg-system
    
    apply_bionic_crd

    if [[ "$@" =~ "--testing" ]]; then
        echo "Running in testing mode"

    # For testing the operator use --development
    elif [[ "$@" =~ "--development" ]]; then
        echo "Not deploying operator use cargo run --bin k8s-operator"
    else
        deploy_bionic_operator
    fi

    deploy_bionic "$address" "$gpu" "$testing"

    echo "When it's ready Bionic-GPT available on http://$address"
    echo "Use k9s to check the status"


}

check_commands_installed() {
    for cmd in "$@"; do
        if ! command -v "$cmd" &> /dev/null; then
            echo "$cmd is not installed"
        else
            echo "$cmd is installed"
        fi
    done
}

# Main script starts here
main() {
    if [[ $# -eq 0 ]]; then
        echo "Usage: $0 {install|reqs|pgadmin}"
        exit 1
    fi

    case "$1" in
        install)
            shift
            install "$@"
            ;;
        reqs)
            check_commands_installed k9s kubectl k3s
            ;;
        pgadmin)
            expose_pgadmin
            ;;
        *)
            echo "Unknown command: $1"
            echo "Usage: $0 {install|reqs|pgadmin}"
            exit 1
            ;;
    esac

    exit 0
}

# Run the script with parameters
main "$@"