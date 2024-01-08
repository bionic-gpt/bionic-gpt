kind delete cluster --name bionic-gpt-cluster
kind create cluster --name bionic-gpt-cluster --config=/workspace/crates/k8s-operator/kind-config.yaml
kind export kubeconfig --name bionic-gpt-cluster
sed -i 's,https://0.0.0.0,https://host.docker.internal,g' ~/.kube/config
kubectl apply -f crates/k8s-operator/bionics.bionic-gpt.com.yaml
kubectl apply -f crates/k8s-operator/bionic.yaml