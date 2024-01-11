+++
title = "Single Sign On"
description = "Single Sign On"
weight = 100
sort_by = "weight"
+++

We use Open ID Connect to allow you to connect Bionic-GPT to any an identity access management (IAM) provider. For example Auth0, Azure, OneLogin, Google and more.

You need to create the following secret in kubernetes and reference it from the `bionic.yaml` we referenced earlier.

Example secret

```yml
apiVersion: v1
kind: Secret
metadata:
  name: oidc-secret
type: Opaque
data:
  client-id: <base64-encoded-client-id>
  client-secret: <base64-encoded-client-secret>
  redirect-uri: <base64-encoded-redirect-uri>
  issuer-url: <base64-encoded-issuer-url>
```

Replace `<base64-encoded-client-id>`, `<base64-encoded-client-secret>`, `<base64-encoded-redirect-uri>`, and `<base64-encoded-issuer-url>` with the base64-encoded values of your actual OIDC provider information. You can use the `echo -n value | base64 command to generate the base64-encoded values.`

```sh
echo -n 'your-client-id' | base64
echo -n 'your-client-secret' | base64
echo -n 'your-redirect-uri' | base64
echo -n 'your-issuer-url' | base64
```

## Connecting the secret to Bionic

```yml
apiVersion: bionic-gpt.com/v1
kind: Bionic
metadata:
  name: bionic-gpt
  namespace: bionic-gpt 
spec:
  replicas: 1 
  version: 1.5.21
  
  # Enterprise Options
  # enterprise-key: EMAIL_US_FOR_A_KEY
  # enterprise-user: your_email@your_work_address.com
  # enterprise-expiry: Expiry date of your key (For trial users)

  # Use by Oauth2 proxy to know where to redirect and also keycloak (if installed)
  # to know how to set the openid configuration
  hostname-url: https://localhost

  # open-id-connect-secret: secret-name

  # Image hashes to protect against supply chain attacks.
  hash-bionicgpt: sha256:5d93122b7e6cb9a1cb4e30d51e624dab95e978049c6e0660847ae0764590924e
  hash-bionicgpt-pipeline-job: sha256:80f1c3fdacad6f62f0a08899a2a25fab701a0515aa9840a0454d5a545c29d293
  hash-bionicgpt-db-migrations: sha256:b3ed62ed8cd595ccd28cb227a8ebfabd3855f8f571344a405780d9affe63a591
```

Uncomment the `# open-id-connect-secret: secret-name` and add your secret name.

Bionic-GPT will not install Keycloak in this case but will use the provided config.