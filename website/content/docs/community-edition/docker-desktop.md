+++
title = "Docker Desktop Installation (Experimental)"
weight = 30
sort_by = "weight"
+++

If you have already installed [Docker Desktop](https://www.docker.com/products/docker-desktop/) then you have easy access to a Kubernetes installation. This should work on Windows, Linux and MacOs including Apple Silicon.

However it's difficult for us to test on all these system hence why we have it down as experimental. If you have any issues please raise it in the [discussions](https://github.com/bionic-gpt/bionic-gpt/discussions).

Go to settings and enable Kubernetes

![Alt text](../docker-desktop.png "Docker Desktop")


## 1. Install the Bionic CLI (MacOS)

```sh
export BIONIC_VERSION={{ version() }}
curl -OL https://github.com/bionic-gpt/bionic-gpt/releases/download/v${BIONIC_VERSION}/bionic-cli-darwin && chmod +x ./bionic-cli-darwin && sudo mv ./bionic-cli-darwin /usr/local/bin/bionic
```

Try it out

```sh
bionic -h
```

## 1. Install the Bionic CLI (Windows)

Windows executables are available here https://github.com/bionic-gpt/bionic-gpt/releases

Try it out

```sh
bionic.exe -h
```

## 1. Install the Bionic CLI (Linux)

```sh
export BIONIC_VERSION={{ version() }}
curl -OL https://github.com/bionic-gpt/bionic-gpt/releases/download/v${BIONIC_VERSION}/bionic-cli-linux && chmod +x ./bionic-cli-linux && sudo mv ./bionic-cli-linux /usr/local/bin/bionic
```

Try it out

```sh
bionic -h
```

## 2. Install the Nginx Ingress Operator

```sh
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/controller-v1.10.0/deploy/static/provider/cloud/deploy.yaml
```

## 3. Install the application into Docker Desktop

```sh
bionic install
```

## 4. Update the Nginx Ingress

```sh
cat <<EOF | kubectl apply -n bionic-gpt -f-
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  annotations:
    kubernetes.io/ingress.class: "nginx"
    # We need to set the buffer size or keycloak won't let you register
    nginx.ingress.kubernetes.io/proxy-buffer-size: "128k"
    # We need toi set this as the max size for document upload
    nginx.ingress.kubernetes.io/proxy-body-size: "50m"
  name: bionic-gpt-ingress
spec:
  rules:
  - http:
      paths:
      - path: /oidc
        pathType: Prefix
        backend:
          service:
            name: keycloak
            port:
              number: 7910
      - path: /
        pathType: Prefix
        backend:
          service:
            name: oauth2-proxy
            port:
              number: 7900
EOF
```

## The Finished Result

After a while of container creation you should see all the pods running and then be able to access Bionic.


![Alt text](../bionic-startup-k9s.png "Bionic K9s")

## Run the User Interface

You can then access the front end from `http://localhost` and you'll be redirected to a registration screen.

## Registration

The first user to register with **BionicGPT** will become the system administrator. The information is kept local to your machine and your data is not sent anywhere.

![Alt text](../initial-screen.png "Start Screen")