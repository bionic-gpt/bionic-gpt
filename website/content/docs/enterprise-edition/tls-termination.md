+++
title = "TLS Termination"
weight = 90
sort_by = "weight"
+++

We can easily add our TLS certificates by adding the certificate to a Kubernetes Secret and referencing it in the Kubernetes Ingress.

```sh
cat <<EOF | kubectl apply -n bionic-gpt -f-
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: bionic-gpt-ingress
spec:
  tls:
    - hosts:
      - yoursubdomain.yourdomain.com
      # This assumes tls-secret exists and the SSL
      # certificate contains a CN for yoursubdomain.yourdomain.com
      secretName: tls-secret
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