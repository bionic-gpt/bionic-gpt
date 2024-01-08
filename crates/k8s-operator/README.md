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

## Bounce the Cluster

`crates/k8s-operator/bounce-cluster.sh`

## Forward Keycloak Port

`kubectl port-forward svc/keycloak 7910:7910`
`kubectl port-forward svc/oauth2-proxy 7900:7900`