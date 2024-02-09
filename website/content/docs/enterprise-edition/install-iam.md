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
export ADMIN_PASSWORD=$(openssl rand -hex 10)
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

Create the secrets.

```sh
echo "apiVersion: v1
kind: Secret
metadata:
  namespace: bionic-gpt
  name: keycloak-secrets
data:
  database-password: ${DATABASE_PASSWORD}
  admin-password: ${ADMIN_PASSWORD}
" > keycloak-secrets.yml
```

And apply it

```sh
kubectl apply -n bionic-gpt -f keycloak-secrets.yml
```

## Create a KeyCloak Deployment

```sh
echo "apiVersion: apps/v1
kind: Deployment
metadata:
  name: keycloak
spec:
  replicas: 1
  selector:
    matchLabels:
      app: keycloak
  template:
    metadata:
      labels:
        app: keycloak
    spec:
      containers:
      - name: keycloak
        image: quay.io/keycloak/keycloak:23.0
        volumeMounts:
        - name: keycloak-config
          mountPath: /opt/keycloak/data/import
        ports:
        - containerPort: 7910
        command:
        args:
          - start-dev
          - --import-realm
          - --http-port=7910
          - --proxy=edge
          - --hostname-strict=false
          - --hostname-strict-https=false
          - --hostname-url=https://localhost/oidc
          - --http-relative-path=/oidc

        env:
        - name: KC_DB
          value: postgres
        - name: KC_DB_PASSWORD
          valueFrom:
            secretKeyRef:
              name: keycloak-secrets
              key: database-password
        - name: KC_DB_USERNAME
          value: keycloak-db-owner
        - name: KC_DB_URL
          value: jdbc:postgresql://keycloak-db-cluster-rw/keycloak
        - name: KEYCLOAK_ADMIN
          value: admin
        - name: KEYCLOAK_ADMIN_PASSWORD
          valueFrom:
            secretKeyRef:
              name: keycloak-secrets
              key: admin-password
        - name: KC_HEALTH_ENABLED
          value: 'true'
      volumes:
      - name: keycloak-config
        configMap:
          name: keycloak-config
---
apiVersion: v1
kind: Service
metadata:
  name: keycloak
spec:
  selector:
    app: keycloak
  ports:
    - protocol: TCP
      port: 80
      targetPort: 7910
  type: LoadBalancer
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: keycloak-config
data:
  keycloak-realm-config.json: |-
    {
      'realm': 'bionic-gpt',
      'registrationAllowed': true,
      'registrationEmailAsUsername': true,
      'enabled': 'true',
      'sslRequired': 'none',
      'clients': [
          {
              'clientId': 'bionic-gpt',
              'clientAuthenticatorType': 'client-secret',
              'secret': '69b26b08-12fe-48a2-85f0-6ab223f45777',
              'redirectUris': [
                  'http://*',
                  'https://*'
              ],
              'protocol': 'openid-connect'
          }
      ]
    }
" > keycloak-deployment.yml
```

And apply it

```sh
kubectl apply -n bionic-gpt -f keycloak-deployment.yml
```

Create the secret so Bionic can see the Open ID Connect provider.

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