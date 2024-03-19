+++
title = "Bare Metal Installation"
weight = 10
sort_by = "weight"
+++

We'll use [K3s](https://k3s.io/) to install Bionic. 

K3s is a highly available, certified Kubernetes distribution designed for production workloads in unattended, resource-constrained, remote locations or inside IoT appliances.

## Install Script

The install script install K3s and then the Postgres operator and finally the Bionic Operator.

```sh
curl -LO https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/install.sh && chmod +x ./install.sh && ./install.sh
```

## The Finished Result

After a while of container creation you should see all th epods running and then be able to access Bionic.


![Alt text](../bionic-startup-k9s.png "Bionic K9s")
