# Install on Amazon Cloud

We can use [https://eksctl.io](https://eksctl.io/) which is the AWS supported way of setting up a K8's cluster (EKS). 

## 1. Get your credentials

```sh
export AWS_ACCESS_KEY_ID=...
export AWS_SECRET_ACCESS_KEY=...
```

## 2. Setup your EKS install config

Copy the below (changing it as necessary) into a file called `cluster.yaml`. You'll need to set the {ACCOUNT_ID} to your account.

```yaml
apiVersion: eksctl.io/v1alpha5
kind: ClusterConfig
metadata:
  name: bionic-gpt
  region: us-east-2
managedNodeGroups:
- name: bionic-gpt
  instanceType: t2.large
  minSize: 2
  maxSize: 4
iam:
  withOIDC: true
  serviceAccounts:
  - metadata:
      name: ebs-csi-controller-sa
      namespace: kube-system
    attachPolicyARNs:
    - "arn:aws:iam::aws:policy/service-role/AmazonEBSCSIDriverPolicy"
    wellKnownPolicies:
      ebsCSIController: true
    roleName: eksctl-cluster-ebs-role
    roleOnly: true
addons:
- name: aws-ebs-csi-driver
  serviceAccountRoleARN: "arn:aws:iam::{ACCOUNT_ID}:role/eksctl-cluster-ebs-role"
```

## Dry Run `eksctl`

First a dry run

```sh
eksctl create cluster --dry-run -f cluster.yaml
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
eksctl create cluster -f cluster.yaml
```

And wait....

And wait....

Finally after 20 minutes or more you'll hopefully have an EKS cluster.

## Accessing the Kubeconfig file

The below will export the kubeconfig to your home directory.

```sh
eksctl utils write-kubeconfig --cluster bionic-gpt --region us-east-2
```

You can then 

```sh
kubectl get nodes
```

Follow the "Install Bionic" guide to continue the installation.