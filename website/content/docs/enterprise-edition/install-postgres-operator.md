+++
title = "Installing the Postgres Operator"
weight = 12
sort_by = "weight"
+++

We need Postgres to persist user data and various other states in the Bionic-GPT system.

There are several ways to install Postgres on Kubernetes or get access to a Postgres database. If you have access to a managed Postgres solution you may be able to use that instead of installing Postgres on Kubernetes.

## Installing the Operator

There are several Postgres Operators, the one we have chosen is [CloudNativePG](https://cloudnative-pg.io/).

Refer to the CloudNativePG documentation to get the latest version to install or go with version 1.22 like below.

```sh
kubectl apply -f \
  https://raw.githubusercontent.com/cloudnative-pg/cloudnative-pg/release-1.22/releases/cnpg-1.22.1.yaml
```

Verify the installation

```sh
$ kubectl get deployment -n cnpg-system cnpg-controller-manager
NAME                      READY   UP-TO-DATE   AVAILABLE   AGE
cnpg-controller-manager   1/1     1            1           2m37s
```

## Create the Kubernetes Namespace

```sh
kubectl create namespace bionic-gpt
```

## Creating Some Password and Secrets

To create a database we'll need some randomness for the database passwords and roles we'll create.

```sh
export APP_DATABASE_PASSWORD=$(openssl rand -hex 10)
export DBOWNER_DATABASE_PASSWORD=$(openssl rand -hex 10)
export READONLY_DATABASE_PASSWORD=$(openssl rand -hex 10)
```

```sh
echo "apiVersion: v1
kind: Secret
type: "kubernetes.io/basic-auth"
metadata:
  namespace: bionic-gpt
  name: db-owner
stringData:
  username: db-owner
  password: ${DBOWNER_DATABASE_PASSWORD}
" > db-owner-secret.yml
```

And apply it

```sh
kubectl apply -n bionic-gpt -f db-owner-secret.yml
```

## Creating a Bionic-GPT Database

Create the configuration for a Bionic-GPT database. Feel free to alter the storage size and the number of instances.

```sh
echo "apiVersion: postgresql.cnpg.io/v1
kind: 'Cluster'
metadata:
  name: 'bionic-db-cluster'
  namespace: bionic-gpt
spec:
  instances: 1
  bootstrap:
    initdb:
      database: bionic-gpt
      owner: db-owner
      secret:
        name: db-owner
      postInitSQL:
        - CREATE ROLE bionic_application LOGIN ENCRYPTED PASSWORD '${APP_DATABASE_PASSWORD}'
        - CREATE ROLE bionic_readonly LOGIN ENCRYPTED PASSWORD '${READONLY_DATABASE_PASSWORD}'
  storage:
    size: '1Gi'
" > database.yml
```

And apply it

```sh
kubectl apply -n bionic-gpt -f database.yml
```

Create the secrets so Bionic can access the database.

```sh
echo "apiVersion: v1
kind: Secret
metadata:
  namespace: bionic-gpt
  name: database-urls
stringData:
  migrations-url: postgres://db-owner:${DBOWNER_DATABASE_PASSWORD}@bionic-db-cluster-rw:5432/bionic-gpt?sslmode=require
  application-url: postgres://bionic_application:${APP_DATABASE_PASSWORD}@bionic-db-cluster-rw:5432/bionic-gpt?sslmode=require
  readonly-url: postgres://bionic_readonly:${READONLY_DATABASE_PASSWORD}@bionic-db-cluster-rw:5432/bionic-gpt?sslmode=require
" > db-secrets.yml
```

And apply it

```sh
kubectl apply -n bionic-gpt -f db-secrets.yml
```

## The end result

Assuming all goes well you should be able to see a postgres database running in your cluster.

![Alt text](../postgres-operator.png "Postgres Operator")