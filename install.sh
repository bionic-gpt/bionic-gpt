#!/bin/bash

# Function to check and install tools if not already installed
install_tools() {
    local tool=$1
    if ! command -v "$tool" &>/dev/null; then
        echo "Installing $tool..."
        if [[ $tool == "kind" ]]; then
            install_kind
        elif [[ $tool == "kubectl" ]]; then
            install_kubectl
        elif [[ $tool == "k9s" ]]; then
            install_k9s
        fi
    else
        echo "$tool Already Installed"
    fi
}

# Function to install Kind
install_kind() {
    curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.22.0/kind-linux-amd64
    chmod +x ./kind
    sudo mv ./kind /usr/local/bin/kind
}

# Function to install kubectl
install_kubectl() {
    sudo apt-get update
    sudo apt-get install -y kubectl
}

# Function to install k9s
install_k9s() {
    curl -Lo ./k9s https://github.com/derailed/k9s/releases/download/v0.24.15/k9s_Linux_x86_64.tar.gz
    tar -xvf k9s_Linux_x86_64.tar.gz
    sudo mv ./k9s /usr/local/bin/k9s
}

# Function to clean up previous cluster and create a new one
create_kind_cluster() {
    kind delete cluster --name bionic-gpt-cluster

    cat <<EOF >kind-config.yaml
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
  kubeadmConfigPatches:
  - |
    kind: InitConfiguration
    nodeRegistration:
      kubeletExtraArgs:
        node-labels: "ingress-ready=true"
  extraPortMappings:
  - containerPort: 443
    hostPort: 443
networking:
  # If we don't do this, then we can't connect on linux
  apiServerAddress: "0.0.0.0"
kubeadmConfigPatchesJSON6902:
- group: kubeadm.k8s.io
  version: v1beta3
  kind: ClusterConfiguration
  patch: |
    - op: add
      path: /apiServer/certSANs/-
      value: host.docker.internal
EOF


    kind create cluster --name bionic-gpt-cluster --config=./kind-config.yaml
    rm ./kind-config.yaml
    kind export kubeconfig --name bionic-gpt-cluster
}

# Function to update kubeconfig for Docker Desktop
update_kubeconfig() {
    sed -i 's,https://0.0.0.0,https://host.docker.internal,g' ~/.kube/config
}

# Function to apply Kubernetes configurations
apply_bionic_crd() {
    kubectl create namespace bionic-gpt
    kubectl apply -n bionic-gpt -f https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/k8s-operator/config/bionics.bionic-gpt.com.yaml
}

# Function to install Postrgres
install_postgres() {
    kubectl apply -f https://raw.githubusercontent.com/cloudnative-pg/cloudnative-pg/release-1.22/releases/cnpg-1.22.1.yaml
}

# Function to generate and apply database secrets
apply_database_secrets() {
    export APP_DATABASE_PASSWORD=$(openssl rand -hex 10)
    export DBOWNER_DATABASE_PASSWORD=$(openssl rand -hex 10)
    export READONLY_DATABASE_PASSWORD=$(openssl rand -hex 10)

    cat <<EOF >db-owner-secret.yml
apiVersion: v1
kind: Secret
type: "kubernetes.io/basic-auth"
metadata:
  namespace: bionic-gpt
  name: db-owner
stringData:
  username: db-owner
  password: ${DBOWNER_DATABASE_PASSWORD}
EOF

    kubectl apply -n bionic-gpt -f db-owner-secret.yml && rm db-owner-secret.yml

    cat <<EOF >db-secrets.yml
apiVersion: v1
kind: Secret
metadata:
  namespace: bionic-gpt
  name: database-urls
stringData:
  migrations-url: postgres://db-owner:${DBOWNER_DATABASE_PASSWORD}@bionic-db-cluster-rw:5432/bionic-gpt?sslmode=require
  application-url: postgres://bionic_application:${APP_DATABASE_PASSWORD}@bionic-db-cluster-rw:5432/bionic-gpt?sslmode=require
  readonly-url: postgres://bionic_readonly:${READONLY_DATABASE_PASSWORD}@bionic-db-cluster-rw:5432/bionic-gpt?sslmode=require
EOF

    kubectl apply -n bionic-gpt -f db-secrets.yml && rm db-secrets.yml

    echo "Waiting for the Postgres Operator to start"
    sleep 20
    kubectl -n cnpg-system wait --timeout=30s --for=condition=ready pod -l app.kubernetes.io/name=cloudnative-pg
}

# Function to deploy PostgreSQL cluster
deploy_postgres_cluster() {
    cat <<EOF >database.yml
apiVersion: postgresql.cnpg.io/v1
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
EOF

    kubectl apply -n bionic-gpt -f database.yml && rm database.yml
}

# Function to generate and apply keycloak secrets
apply_keycloak_secrets() {
    export DATABASE_PASSWORD=$(openssl rand -hex 10)
    export ADMIN_PASSWORD=$(openssl rand -hex 10)

    cat <<EOF >keycloak-db-secret.yml
apiVersion: v1
kind: Secret
type: "kubernetes.io/basic-auth"
metadata:
  namespace: bionic-gpt
  name: keycloak-db-owner
stringData:
  username: keycloak-db-owner
  password: ${DATABASE_PASSWORD}
EOF

    kubectl apply -n bionic-gpt -f keycloak-db-secret.yml && rm keycloak-db-secret.yml

    cat <<EOF >keycloak-database.yml
apiVersion: postgresql.cnpg.io/v1
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
EOF

    kubectl apply -n bionic-gpt -f keycloak-database.yml && rm keycloak-database.yml

    cat <<EOF >keycloak-secrets.yml
apiVersion: v1
kind: Secret
metadata:
  namespace: bionic-gpt
  name: keycloak-secrets
data:
  database-password: ${DATABASE_PASSWORD}
  admin-password: ${ADMIN_PASSWORD}
EOF

    kubectl apply -n bionic-gpt -f keycloak-secrets.yml && rm keycloak-secrets.yml
}

# Function to deploy keycloak
deploy_keycloak() {
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


    # Point to the ip address
    sed -i "s/localhost/$1/g" ./keycloak-deployment.yml

    kubectl apply -n bionic-gpt -f keycloak-deployment.yml && rm keycloak-deployment.yml
}

# Function to check if Docker-in-Docker parameter is supplied and update kubeconfig
check_docker_in_docker() {
    if [[ "$@" =~ "--docker-in-docker" ]]; then
        update_kubeconfig
    fi
}

preload_images() {
    echo "Preloading unstructured and embeddings for a faster startup (This takes a long time)"
    kind --name bionic-gpt-cluster load docker-image downloads.unstructured.io/unstructured-io/unstructured-api:4ffd8bc
    kind --name bionic-gpt-cluster load docker-image ghcr.io/bionic-gpt/bionicgpt-embeddings-api:cpu-0.6
}

deploy_bionic_operator() {
    kubectl apply -f https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/k8s-operator/config/bionic-operator.yaml
}

deploy_bionic() {

    curl -LO https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/crates/k8s-operator/config/bionic.yaml

    # Point to the ip address
    sed -i "s/localhost/$1/g" ./bionic.yaml
    sed -i "s/# pgadmin/pgadmin/g" ./bionic.yaml
    sed -i "s/# gpu: true/gpu: $2/g" ./bionic.yaml

    kubectl apply -f ./bionic.yaml
    rm ./bionic.yaml
}

apply_oauth2_proxy_secrets() {

    export COOKIE_SECRET=$(dd if=/dev/urandom bs=32 count=1 2>/dev/null | base64 | tr -d -- '\n' | tr -- '+/' '-_' ; echo)

    echo "apiVersion: v1
kind: Secret
metadata:
  name: oidc-secret
stringData:
  client-id: bionic-gpt
  client-secret: 69b26b08-12fe-48a2-85f0-6ab223f45777
  redirect-uri: https://localhost/oauth2/callback
  issuer-url: http://keycloak/oidc/realms/bionic-gpt
  cookie-secret: ${COOKIE_SECRET}
" > oidc-secret.yml

    sed -i "s/localhost/$1/g" ./oidc-secret.yml

    kubectl apply -n bionic-gpt -f oidc-secret.yml && rm oidc-secret.yml
}

install_ingress() {
    kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml
}

connect_ingress() {

    cat <<EOF | kubectl apply -n bionic-gpt -f-
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  annotations:
    # We need to set the buffer size or keycloak won't let you register
    nginx.ingress.kubernetes.io/proxy-buffer-size: "128k"
    # We need toi set this as the max size for document upload
    nginx.ingress.kubernetes.io/proxy-body-size: "50m"
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

}

apply_pgadmin_secrets() {

    export PASSWORD=$(dd if=/dev/urandom bs=32 count=1 2>/dev/null | base64 | tr -d -- '\n' | tr -- '+/' '-_' ; echo)

    echo "apiVersion: v1
kind: Secret
metadata:
  name: pgadmin-secret
stringData:
  email: test@test.com
  password: ${PASSWORD}
" > pgadmin-secret.yml

    kubectl apply -n bionic-gpt -f pgadmin-secret.yml && rm pgadmin-secret.yml
}

# Main function
main() {

    if [[ "$@" =~ "--localhost" ]]; then
        address="localhost"
    else
        address=$(hostname -I | awk '{print $1}')
    fi

    if [[ "$@" =~ "--gpu" ]]; then
        gpu="true"
    else
        gpu="false"
    fi

    # Install tools if not installed
    install_tools "kind"
    install_tools "kubectl"
    install_tools "k9s"

    # Execute each step
    create_kind_cluster
    check_docker_in_docker "$@"
    apply_bionic_crd
    install_postgres
    apply_database_secrets
    deploy_postgres_cluster
    apply_keycloak_secrets
    deploy_keycloak "$address"
    install_ingress
    preload_images
    deploy_bionic_operator
    apply_oauth2_proxy_secrets "$address"
    apply_pgadmin_secrets
    deploy_bionic "$address $gpu"
    connect_ingress

    echo "Bionic-GPT available on https://$address"
}

# Run the script with parameters
main "$@"