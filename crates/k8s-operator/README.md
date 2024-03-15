## To create the CRD in Kubernetes

`kubectl apply -f crates/k8s-operator/config/bionics.bionic-gpt.com.yaml`

## To Verify

`kubectl get crds`

## Install Bionic

`kubectl apply -f crates/k8s-operator/config/bionic.yaml`

## Preload images

Do we need this?

`kind --name bionic-gpt-cluster load docker-image downloads.unstructured.io/unstructured-io/unstructured-api:4ffd8bc`

`kind --name bionic-gpt-cluster load docker-image ghcr.io/bionic-gpt/llama-2-7b-chat:1.0.4`

`kind --name bionic-gpt-cluster load docker-image ghcr.io/bionic-gpt/bionicgpt-embeddings-api:cpu-0.6`

## Run the operator

`cargo run --bin k8s-operator`

## Remove Bionic

`kubectl delete -f crates/k8s-operator/config/bionic.yaml`

## To remove the CRD in Kubernetes

`kubectl delete -f crates/k8s-operator/config/bionics.bionic-gpt.com.yaml`

## Remove the operator

`kubectl delete -f crates/k8s-operator/config/bionic-operator.yaml`

## Bounce the Cluster

`install.sh --docker-in-docker`

## Forward Keycloak Port

`kubectl port-forward svc/keycloak 7910:7910`
`kubectl port-forward svc/oauth2-proxy 7900:7900`