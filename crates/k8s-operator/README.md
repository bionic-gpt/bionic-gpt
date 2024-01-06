## To create the CRD in Kubernetes

`kubectl apply -f crates/k8s-operator/bionics.bionic-gpt.com.yaml`

## To Verify

`kubectl get crds`

## Run the operator

`cargo run --bin k8s-operator`