## Infrastructure as Code

In Bionic the Kubernetes Opoerator does the heavy lifting when it comes to deployment. However you do need a Kubernetes cluster.

This folder holds the config we use to setup a cluster for our use.

The code that executes the config is in the `Earthfile` and more or less folloes the EKS instructions from [Install on AWS](https://bionic-gpt.com/docs/enterprise-edition/aws/)

## Cluster Teardown

```
earthly +drop-eks-cluster --AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID --AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY
```

## Create Cluster and Deploy Bionic

```
earthly +create-eks-cluster --AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID --AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY --TUNNEL_TOKEN=$TUNNEL_TOKEN
```