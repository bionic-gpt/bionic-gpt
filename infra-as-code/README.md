## Infrastructure as Code

In Bionic the Kubernetes Operator does the heavy lifting when it comes to deployment. However you do need a Kubernetes cluster.

This folder holds the config we use to setup a cluster for our use on Amazon AWS.

The code that executes the config is in the `Earthfile` and more or less follows the EKS instructions from [Install on AWS](https://bionic-gpt.com/docs/enterprise-edition/aws/)

## Cluster Teardown

```
earthly +drop-eks-cluster --AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID --AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY
```

## Create Cluster and Deploy Bionic

```
earthly +create-eks-cluster --AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID --AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY --AWS_ACCOUNT_ID=$AWS_ACCOUNT_ID --TOKEN=$TOKEN
```

## Cloudflare

We install a cloudflare tunnel as a way to get ingress inot our cluster via the domain name.

## Access Cluster

```sh
eksctl utils write-kubeconfig --cluster bionic-gpt --region us-east-2
```