+++
title = "Keycloak Administration"
weight = 60
sort_by = "weight"
+++

We include [Keycloak](https://www.keycloak.org/) as an identity and access management system to get you started quickly.

We do recommend you connect Bionic with your own IAM system, however if you want to get admin access to Keycloak follow this guide.

## Getting the Admin Password

The admin interface is available on `http(s)://YOUR_DOMAIN_OR_IP/oidc`

To get the password

```sh
kubectl get secret -n bionic-gpt keycloak-secrets -o jsonpath='{.data.admin-password}' | base64 --decode
```

Then you can log in with username `admin` and the password from above.