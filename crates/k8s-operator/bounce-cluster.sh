kind delete cluster --name bionic-gpt-cluster
kind create cluster --name bionic-gpt-cluster --config=/workspace/crates/k8s-operator/config/kind-config.yaml
kind export kubeconfig --name bionic-gpt-cluster
sed -i 's,https://0.0.0.0,https://host.docker.internal,g' ~/.kube/config
kubectl create namespace bionic-gpt
kubectl apply -n bionic-gpt -f /workspace/crates/k8s-operator/config/bionics.bionic-gpt.com.yaml
kubectl apply -f /workspace/crates/k8s-operator/config/bionic.yaml

## Install Postrgres
kubectl apply -f https://raw.githubusercontent.com/cloudnative-pg/cloudnative-pg/release-1.22/releases/cnpg-1.22.1.yaml

export APP_DATABASE_PASSWORD=$(openssl rand -hex 10)
export DBOWNER_DATABASE_PASSWORD=$(openssl rand -hex 10)
export READONLY_DATABASE_PASSWORD=$(openssl rand -hex 10)

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

kubectl apply -n bionic-gpt -f db-owner-secret.yml && rm db-owner-secret.yml

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


kubectl apply -n bionic-gpt -f db-secrets.yml && rm db-secrets.yml

sleep 20

kubectl -n cnpg-system wait --timeout=30s --for=condition=ready pod -l app.kubernetes.io/name=cloudnative-pg

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
      postInitApplicationSQL:
        - CREATE EXTENSION IF NOT EXISTS vector
  storage:
    size: '1Gi'
" > database.yml

kubectl apply -n bionic-gpt -f database.yml && rm database.yml

export DATABASE_PASSWORD=$(openssl rand -hex 10)
export ADMIN_PASSWORD=$(openssl rand -hex 10)

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

kubectl apply -n bionic-gpt -f keycloak-db-secret.yml && rm keycloak-db-secret.yml

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

kubectl apply -n bionic-gpt -f keycloak-database.yml && rm keycloak-database.yml

echo "apiVersion: v1
kind: Secret
metadata:
  namespace: bionic-gpt
  name: keycloak-secrets
data:
  database-password: ${DATABASE_PASSWORD}
  admin-password: ${ADMIN_PASSWORD}
" > keycloak-secrets.yml

kubectl apply -n bionic-gpt -f keycloak-secrets.yml && rm keycloak-secrets.yml

kubectl -n bionic-gpt wait --for=condition=ready pod -l cnpg.io/cluster=bionic-db-cluster
kubectl -n bionic-gpt wait --timeout=30s --for=condition=ready pod -l cnpg.io/cluster=keycloak-db-cluster

echo 'apiVersion: apps/v1
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
        volumeMounts:
        - name: keycloak-config
          mountPath: /opt/keycloak/data/import
        ports:
        - containerPort: 7910
        env:
        #- name: KC_DB
        #  value: postgres
        #- name: KC_DB_PASSWORD
        #  valueFrom:
        #    secretKeyRef:
        #      name: keycloak-secrets
        #      key: database-password
        #- name: KC_DB_USERNAME
        #  value: keycloak-db-owner
        #- name: KC_DB_URL
        #  value: jdbc:postgresql://keycloak-db-cluster-rw:5432/keycloak
        - name: KEYCLOAK_ADMIN
          value: admin
        - name: KEYCLOAK_ADMIN_PASSWORD
          valueFrom:
            secretKeyRef:
              name: keycloak-secrets
              key: admin-password
        - name: KC_HEALTH_ENABLED
          value: "true"
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
  type: ClusterIP
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: keycloak-config
data:
  realm.json: |
    {
      "realm": "bionic-gpt",
      "registrationAllowed": "true",
      "registrationEmailAsUsername": "true",
      "enabled": "true",
      "sslRequired": "none",
      "clients": [
        {
          "clientId": "bionic-gpt",
          "clientAuthenticatorType": "client-secret",
          "secret": "69b26b08-12fe-48a2-85f0-6ab223f45777",
          "redirectUris": [
            "http://*",
            "https://*"
          ],
          "protocol": "openid-connect"
        }
      ]
    }

' > keycloak-deployment.yml

kubectl apply -n bionic-gpt -f keycloak-deployment.yml && rm keycloak-deployment.yml


echo "apiVersion: v1
kind: Secret
metadata:
  name: oidc-secret
stringData:
  client-id: bionic-gpt
  client-secret: 69b26b08-12fe-48a2-85f0-6ab223f45777
  redirect-uri: https://localhost/oauth2/callback
  issuer-url: http://keycloak/oidc/realms/bionic-gpt
  cookie-secret: OQINaROshtE9TcZkNAm-5Zs2Pv3xaWytBmc5W7sPX7w=
" > oidc-secret.yml

kubectl apply -n bionic-gpt -f oidc-secret.yml && rm oidc-secret.yml