+++
title = "Installing Bionic"
weight = 60
sort_by = "weight"
+++

We can now use the Bionic-GPT operator to install the Deployments, Services, ConfigMaps and secrets that make Bionic-GPT work.

```sh
kubectl apply -n bionic-gpt -f https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/k8s-operator/bionic.yaml
```

## Bionic-GPT Starting up on Kubernetes

Using `k9s` move to the `bionic-gpt` namespace (Type `:` then ns).

You will see the images downloading and the progress as the containers start.

![Alt text](../bionic-startup-k9s.png "Oauth2 Proxy")

## Installing Ingress

```sh
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml
```

## Apply the Ingress to our deployment

```sh
cat <<EOF | kubectl apply -n bionic-gpt -f-
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: bionic-gpt-ingress
  annotations:
    kubernetes.io/ingress.class: "nginx"
    nginx.ingress.kubernetes.io/rewrite-target: /
spec:
  rules:
  - http:
      paths:
      - path: /realm
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

## Accessing Bionic

Bionic-GPT will now be available on `http://localhost`

