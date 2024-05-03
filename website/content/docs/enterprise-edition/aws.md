+++
title = "Installation on AWS"
weight = 35
sort_by = "weight"
+++

We can use https://eksctl.io which is the AWS supported way of setting up a K8's cluster (EKS). 

## 1. Get your credentials

```sh
export AWS_ACCESS_KEY_ID=...
export AWS_SECRET_ACCESS_KEY=...
```

## 2. Setup your EKS install config

Copy the below (changing it as necessary) into a file called `cluster.yaml`

```yaml
apiVersion: eksctl.io/v1alpha5
kind: ClusterConfig
metadata:
  name: devops-catalog
  region: us-east-2
managedNodeGroups:
- name: bionic-gpt
  instanceType: t2.small
  minSize: 3
  maxSize: 6
  spot: true
```

## Dry Run `eksctl`

First a dry run

```sh
docker run -v $(pwd)/:/config \
    -e AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID \
    -e AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY \
    --rm -it public.ecr.aws/eksctl/eksctl create cluster \
    --dry-run -f /config/cluster.yaml
```

You should see something like

```sh
accessConfig:
  authenticationMode: API_AND_CONFIG_MAP
apiVersion: eksctl.io/v1alpha5
availabilityZones:
- us-east-2a
- us-east-2b
- us-east-2c
 ....
 ....
```

## Now Create the Cluster

Run the command without the `--dry-run`.

```sh
docker run -v $(pwd)/:/config \
    -e AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID \
    -e AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY \
    --rm -it public.ecr.aws/eksctl/eksctl create cluster \
    -f /config/cluster.yaml
```