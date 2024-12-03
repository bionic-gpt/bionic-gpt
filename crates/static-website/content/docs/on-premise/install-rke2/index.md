# Why RKE2?

RKE2, also known as RKE Government, is Rancher's next-generation Kubernetes distribution.

It is a fully conformant Kubernetes distribution that focuses on security and compliance within the U.S. Federal Government sector.

To meet these goals, RKE2 does the following:

- Provides defaults and configuration options that allow clusters to pass the CIS Kubernetes Benchmark v1.6 or v1.23 with minimal operator intervention
- Enables FIPS 140-2 compliance
- Regularly scans components for CVEs using trivy in our build pipeline

### 1. Install RKE2

```sh
curl -sfL https://get.rke2.io | sudo sh -
sudo systemctl enable rke2-server.service
sudo systemctl start rke2-server.service
```

### 2. Install K9s (Optional)

```sh
curl -L -s https://github.com/derailed/k9s/releases/download/v0.24.15/k9s_Linux_x86_64.tar.gz | tar xvz -C /tmp
sudo mv /tmp/k9s /usr/local/bin
rm -rf k9s_Linux_x86_64.tar.gz
```

### 3. Check your RKE2 install

```sh
mkdir -p ~/.kube
sudo cp /etc/rancher/rke2/rke2.yaml ~/.kube/config
sudo chmod 644 ~/.kube/config
kubectl get pods
# No resources found in default namespace.
```

### 4. Install Local Path Provisioner

```sh
kubectl apply -f https://raw.githubusercontent.com/rancher/local-path-provisioner/v0.0.24/deploy/local-path-storage.yaml
```

### 5. Install the Bionic CLI

```sh
export BIONIC_VERSION={{ version() }}
curl -OL https://github.com/bionic-gpt/bionic-gpt/releases/download/${BIONIC_VERSION}/bionic-cli-linux && chmod +x ./bionic-cli-linux && sudo mv ./bionic-cli-linux /usr/local/bin/bionic
```

Try it out

```sh
bionic -V
```

### 6. Install the application into RKE2

```sh
bionic install
```

If you get the error below then wait a bit longer. The cluster is still coming up.

```sh
Error: ApiError: "service unavailable\n": Failed to parse error data (ErrorResponse { status: "503 Service Unavailable", message: "\"service unavailable\\n\"", reason: "Failed to parse error data", code: 503 })
```

## The Finished Result

After a while of container creation you should see all the pods running and then be able to access Bionic.


![Alt text](bionic-startup-k9s.png "Bionic K9s")

## Run the User Interface

You can then access the front end from `http://localhost` and you'll be redirected to a registration screen.

## Registration

The first user to register with **BionicGPT** will become the system administrator. The information is kept local to your machine and your data is not sent anywhere.

![Alt text](/landing-page/bionic-console.png "Start Screen")

## Uninstall Bionic

First we can remove K3s entirely. K3s comes with it's own uninstall script.

```sh
sudo rke2-uninstall.sh
```

Then you can remove the bionic cli

```sh
sudo rm /usr/local/bin/bionic
```

And also remove k9s if you want to.

```sh
sudo rm /usr/local/bin/k9s
```
