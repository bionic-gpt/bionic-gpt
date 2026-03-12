# Installation Prerequisite

Private cloud installs are a two step process:

1. Get Kubernetes running (K3s, EKS, GKE).
2. Install Bionic with the Stack CLI.

This page covers step 2. All private cloud install options use Stack once Kubernetes is ready.

## 1. Install Stack CLI

```sh
curl -fsSL https://stack-cli.com/install.sh | bash
```

## 2. Bootstrap Stack in the cluster

```sh
stack init
```

This installs CloudNativePG, Keycloak, ingress, the Stack controller, and the Stack CRDs.

## 3. Deploy Bionic with Stack

```sh
curl -fsSL https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/infra-as-code/stack.yaml \
  -o bionic.stack.yaml
```

```sh
stack deploy --manifest bionic.stack.yaml --profile dev
```

If you get a "service unavailable" error, wait a bit longer for the cluster to finish starting.

```sh
Error: ApiError: "service unavailable\n": Failed to parse error data (ErrorResponse { status: "503 Service Unavailable", message: "\"service unavailable\\n\"", reason: "Failed to parse error data", code: 503 })
```
