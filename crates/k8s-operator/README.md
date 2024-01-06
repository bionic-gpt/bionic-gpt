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