+++
title = "Installing Identity and Access Management"
weight = 14
sort_by = "weight"
+++

We use Open ID Connect to allow you to connect Bionic-GPT to any an identity access management (IAM) provider. For example Auth0, Azure, OneLogin, Google and more.

If you don't have an IAM solution yet, go to the **Installing KeyCloak** section.

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

## Installing KeyCloak

If you don't have access to an IAM solution you can install [KeyCloak](https://www.keycloak.org/) which is an open source identity and access management system.

We'll need to create a database to hold KeyCloak data.

```sh
export DATABASE_PASSWORD=$(openssl rand -hex 10)
```

```sh
echo "apiVersion: v1
kind: Secret
type: "kubernetes.io/basic-auth"
metadata:
  namespace: bionic-gpt
  name: keycloak-db-owner
stringData:
  username: keycloak-db-owner
  password: ${DATABASE_PASSWORD}
" > keycloak-db-secret.yml
```

And apply it

```sh
kubectl apply -n bionic-gpt -f keycloak-db-secret.yml
```

## Creating a Keycloak Database

Create the configuration for a Bionic-GPT database. Feel free to alter the storage size and the number of instances.

```sh
echo "apiVersion: postgresql.cnpg.io/v1
kind: 'Cluster'
metadata:
  name: 'keycloak-db-cluster'
  namespace: bionic-gpt
spec:
  instances: 1
  bootstrap:
    initdb:
      database: keycloak
      owner: keycloak-db-owner
      secret:
        name: keycloak-db-owner
  storage:
    size: '1Gi'
" > keycloak-database.yml
```

And apply it

```sh
kubectl apply -n bionic-gpt -f keycloak-database.yml
```

Create the secrets so Keycloak can access the database.

```sh
echo "apiVersion: v1
kind: Secret
metadata:
  namespace: bionic-gpt
  name: keycloak-database-url
stringData:
  migrations-url: postgres://keycloak-db-owner:${DATABASE_PASSWORD}@keycloak-db-cluster-rw:5432/bionic-gpt?sslmode=require
" > keycloak-db-secrets.yml
```

And apply it

```sh
kubectl apply -n bionic-gpt -f keycloak-db-secrets.yml
```

## Create a KeyCloak Deployment