+++
title = "Quick Installation (Linux)"
weight = 20
sort_by = "weight"
+++

## K3s Installation

To run Bionic we'll install a very lightweight Kubernetes onto our system using [K3s](https://k3s.io/)

### 1. Install K3s

```sh
sudo curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC='server --write-kubeconfig-mode="644"' sh -
mkdir -p ~/.kube
cp /etc/rancher/k3s/k3s.yaml ~/.kube/config && sed -i "s,127.0.0.1,$(hostname -I | awk '{print $1}'),g" ~/.kube/config
```

### 2. Install K9s (Optional)

```sh
curl -L -s https://github.com/derailed/k9s/releases/download/v0.24.15/k9s_Linux_x86_64.tar.gz | tar xvz -C /tmp
sudo mv /tmp/k9s /usr/bin
rm -rf k9s_Linux_x86_64.tar.gz
```

### 3. Check your K3s install

```sh
kubectl get pods
# No resources found in default namespace.
```

## 4. Install the Bionic CLI

```sh
export BIONIC_VERSION=1.6.47
curl -OL https://github.com/bionic-gpt/bionic-gpt/releases/download/v${BIONIC_VERSION}/bionic-cli-linux && chmod +x ./bionic-cli-linux && sudo mv ./bionic-cli-linux /usr/local/bin/bionic
```

Try it out

```sh
bionic -V
```

## 5. Install the application into K3s

```sh
bionic install
```

## The Finished Result

After a while of container creation you should see all the pods running and then be able to access Bionic.


![Alt text](../bionic-startup-k9s.png "Bionic K9s")

## Run the User Interface

You can then access the front end from `http://localhost` and you'll be redirected to a registration screen.

## Registration

The first user to register with **BionicGPT** will become the system administrator. The information is kept local to your machine and your data is not sent anywhere.

![Alt text](../initial-screen.png "Start Screen")
