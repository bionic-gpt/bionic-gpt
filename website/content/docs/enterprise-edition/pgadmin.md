+++
title = "Integrate pgAdmin"
weight = 100
sort_by = "weight"
+++

## Enable from the Operator

Uncomment the `pgadmin` section and set it to true.

```yaml
apiVersion: bionic-gpt.com/v1
kind: Bionic
metadata:
  name: bionic-gpt
  namespace: bionic-gpt 
spec:
  replicas: 1 
  version: 1.6.18
  
  # PgAdmin - Uncomment to install PgAdmin
  pgadmin: true
```

Apply the configuration

```sh
kubectl apply -f bionic.yaml
```

## Getting the pgAdmin Logon Password

To get the login credentials

```sh
kubectl get secret -n bionic-gpt pgadmin -o jsonpath='{.data.email}' | base64 --decode
kubectl get secret -n bionic-gpt pgadmin -o jsonpath='{.data.passord}' | base64 --decode
```

## Accessing the Database

```sh
kubectl get secret -n bionic-gpt database-urls -o jsonpath='{.data.readonly-url}' | base64 --decode
```