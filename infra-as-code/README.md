## Example Infra as Code

This is an example of how to setup a Kubernetes cluster ready for a BionicGPT deployment.

Instructions are here https://bionic-gpt.com/docs/production/introduction/

## Create the cluster

```sh
kind create cluster --name bionic-gpt-cluster --config=config.yaml
kind export kubeconfig --name bionic-gpt-cluster
sed -i 's,https://0.0.0.0,https://host.docker.internal,g' ~/.kube/config
```

## Run Pulumi 

```sh
pulumi up --stack k8-cluster
```

## Connect to the database

```sh
kubectl port-forward service/bionic-gpt-db-cluster-rw 5455:5432 --namespace=bionic-gpt
```

```sh
kubectl get secret database-urls -o jsonpath='{.data.application-url}' --namespace bionic-gpt | base64 --decode
```

## Run Pulumi for the app

From the root folder

```
pulumi up --stack bionic-gpt-app
```

## Drop the Kind Cluster

```sh
kind delete cluster --name bionic-gpt-cluster
pulumi stack rm k8-cluster --force
```