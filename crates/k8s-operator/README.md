## To create the CRD in Kubernetes

`kubectl apply -f crates/k8s-operator/bionics.bionic-gpt.com.yaml`

## To Verify

`kubectl get crds`

## Install Bionic

`kubectl apply -f crates/k8s-operator/bionic.yaml`

## Run the operator

`cargo run --bin k8s-operator`

## Remove Bionic

`kubectl delete -f crates/k8s-operator/bionic.yaml`

## To remove the CRD in Kubernetes

`kubectl delete -f crates/k8s-operator/bionics.bionic-gpt.com.yaml`

## Remove the cluster

kind delete cluster --name bionic-gpt-cluster

## Create the cluster

`kind create cluster --name bionic-gpt-cluster --config=crates/k8s-operator/kind-config.yaml`

`kind export kubeconfig --name bionic-gpt-cluster`

`sed -i 's,https://0.0.0.0,https://host.docker.internal,g' ~/.kube/config`