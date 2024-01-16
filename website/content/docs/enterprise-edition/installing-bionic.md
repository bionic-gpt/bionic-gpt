+++
title = "Installing Bionic"
weight = 60
sort_by = "weight"
+++

We can now use the Bionic-GPT operator to install the Deployments, Services, ConfigMaps and secrets that make Bionic-GPT work.

```sh
kubectl apply -n bionic-gpt -f https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/k8s-operator/config/bionic.yaml
```

## Bionic-GPT Starting up on Kubernetes

Using `k9s` move to the `bionic-gpt` namespace (Type `:` then ns).

You will see the images downloading and the progress as the containers start.

![Alt text](../bionic-startup-k9s.png "Oauth2 Proxy")

## Installing Ingress

```sh
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml
```

And wait for ingress to come up

```sh
kubectl wait --namespace ingress-nginx \
  --for=condition=ready pod \
  --selector=app.kubernetes.io/component=controller \
  --timeout=90s
```

## Apply the Ingress to our deployment

```sh
cat <<EOF | kubectl apply -n bionic-gpt -f-
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  annotations:
    # We need to set the buffer size or keycloak won't let you register
    nginx.ingress.kubernetes.io/proxy-buffer-size: "128k"
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

## Accessing Bionic

Bionic-GPT will now be available on `https://localhost` you will see a TLS warning. You can click on advance.

![Alt text](../tls-warning.png "TLS Warning")

## It's Installed

If you see this screen Bionic-GPT is working

![Alt text](../keycloak.png "Keycloak")