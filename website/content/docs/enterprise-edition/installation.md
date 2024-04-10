+++
title = "Installing Bionic"
weight = 30
sort_by = "weight"
+++

## Step 0: Setup

Before anything else, we need to ensure you have access to modern Kubernetes cluster and a functioning kubectl command on your local machine. (If you don’t already have a Kubernetes cluster, one easy option is to run one on your local machine. There are many ways to do this, including kind, k3d, Docker for Desktop, and more.)

Validate your Kubernetes setup by running:

```sh
kubectl version
```
The bionic installer simplifies a lot of the setup. To install it run the following.

## Step 1: Install the CLI

```sh
export BIONIC_VERSION=1.6.35
curl -OL https://github.com/bionic-gpt/bionic-gpt/releases/download/v${BIONIC_VERSION}/bionic-cli-linux && chmod +x ./bionic-cli-linux && sudo mv ./bionic-cli-linux /usr/local/bin/bionic
```

Check the installation

```sh
bionic -h
```

## Step 2: Run the Install

The following will install `k3s` as our kubernetes engine and then install bionic into the cluster. It will also install `k9s` which is a terminal UI for Kubernetes.

```sh
bionic install --pgadmin
```

Note you can skip the `--pgadmin` if you don't want [pgAdmin](https://www.pgadmin.org/) installed.

You can optionally install [k9s](https://k9scli.io/) which is a great way to get insight into your cluster.

```sh
curl -L -s https://github.com/derailed/k9s/releases/download/v0.32.4/k9s_Linux_amd64.tar.gz | tar xvz -C /tmp && sudo mv /tmp/k9s /usr/local/bin && rm -rf k9s_Linux_amd64.tar.gz
```

## Step 3: The Finished Result

and then.

```sh
k9s
```

After a while of container creation you should see all the pods running and then be able to access Bionic.


![Alt text](../bionic-startup-k9s.png "Bionic K9s")

## Step 4: Run the User Interface

You can then access the front end from `http://{YOUR_IP_ADDRESS}` and you'll be redirected to a registration screen.

To get your ip address

```sh
hostname -I | awk '{print $1}'
```

## Step 5: Registration

The first user to register with **BionicGPT** will become the system administrator. The information is kept local to your machine and your data is not sent anywhere.

![Alt text](../initial-screen.png "Start Screen")
