# Install via the Bionic Operator

In environments where you don't want to use the Bionic installer you can install with the operator directly.

## Prerequisites

Bionic requires the following operators to function. 

[CloudNativePG a Kubernetes operator for Postgres](https://cloudnative-pg.io/)

If you want Ingress (i.e. You're not using Cloudflare Tunnel) then you'll also need the Nginx Ingress Operator

[NGINX and NGINX Plus Ingress Controllers for Kubernetes](https://github.com/nginx/kubernetes-ingress)

### 1. Apply the CustomResourceDefinition (CRD)

```bash
kubectl create namespace bionic-system
```

And apply the CRD to the cluster (Use the example below)

```bash
kubectl apply -n bionic-system -f bionic-crd.yaml
```

### Example CRD YAML (`bionic-crd.yaml`)

```yaml
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: bionics.bionic-gpt.com
spec:
  group: bionic-gpt.com
  versions:
    - name: v1
      served: true
      storage: true
      schema:
        openAPIV3Schema:
          type: object
          properties:
            spec:
              type: object
              properties:
                replicas:
                  type: integer
                version:
                  type: string
                gpu:
                  type: boolean
                saas:
                  type: boolean
                disable_ingress:
                  type: boolean
                pgadmin:
                  type: boolean
                observability:
                  type: boolean
                development:
                  type: boolean
                testing:
                  type: boolean
                bionic_db_disk_size:
                  type: integer
                keycloak_db_disk_size:
                  type: integer
                hostname-url:
                  type: string
                hash-bionicgpt:
                  type: string
                hash-bionicgpt-rag-engine:
                  type: string
                hash-bionicgpt-db-migrations:
                  type: string
      subresources:
        status: {}
  scope: Namespaced
  names:
    plural: bionics
    singular: bionic
    kind: Bionic
    shortNames:
      - bio
```

### 2. Install Roles and Service Account

Before deploying the operator, you need to set up the necessary roles and service accounts to grant the operator the required permissions.

Create a `ServiceAccount`, `ClusterRole`, and `ClusterRoleBinding` YAML:

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: bionic-operator-sa
  namespace: bionic-system
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: bionic-operator-role
rules:
  - apiGroups: ["*"]
    resources: ["*"]
    verbs: ["*"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: bionic-operator-binding
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: bionic-operator-role
subjects:
  - kind: ServiceAccount
    name: bionic-operator-sa
    namespace: bionic-system
```

Apply the roles and service account:

```bash
kubectl apply -n bionic-system -f bionic-roles.yaml
```

### 3. Deploy the Operator

You need to deploy the operator (compiled from the Rust code) as a container in your Kubernetes cluster.

Create a `Deployment` YAML for the operator:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: bionic-operator
  namespace: bionic-system
spec:
  replicas: 1
  selector:
    matchLabels:
      app: bionic-operator
  template:
    metadata:
      labels:
        app: bionic-operator
    spec:
      serviceAccountName: bionic-operator-sa
      containers:
        - name: bionic-operator
          image: ghcr.io/bionic-gpt/bionicgpt-k8s-operator:1.9.2
          imagePullPolicy: Always
          env:
            - name: RUST_LOG
              value: info
          resources:
            limits:
              memory: "256Mi"
              cpu: "500m"
```

Apply this Deployment:

```bash
kubectl apply -n bionic-system -f bionic-operator-deployment.yaml
```

### 4. Create a Custom Resource

Now that the operator is running, create an instance of the `Bionic` resource:

First a namespace

```bash
kubectl create namespace bionic-gpt
```

Create a `bionic-instance.yaml` with the example configuration below.

```yaml
apiVersion: bionic-gpt.com/v1
kind: Bionic
metadata:
  name: bionic-gpt
spec:
  replicas: 3
  version: "1.9.2"
  gpu: true
  saas: false
  disable_ingress: false
  pgadmin: true
  observability: true
  development: false
  testing: false
  bionic_db_disk_size: 20
  keycloak_db_disk_size: 10
  hostname-url: "https://example-bionic.com"
  hash-bionicgpt: "abc123"
  hash-bionicgpt-rag-engine: "def456"
  hash-bionicgpt-db-migrations: "ghi789"
```

Apply this resource:

```bash
kubectl apply -n bionic-gpt -f bionic-instance.yaml
```

### 5. Verify the Operator is Managing the Resource

Check if the `Bionic` resource has been created:

```bash
kubectl get -n bionic-gpt bionics
```

Inspect logs of the operator to verify that it's processing the resource:

```bash
kubectl logs -l app=bionic-operator -n bionic-system
```

### 6. Clean Up (Optional)

If you want to remove everything:

```bash
kubectl delete -f bionic-instance.yaml
kubectl delete -f bionic-operator-deployment.yaml
kubectl delete -f bionic-roles.yaml
kubectl delete -f bionic-crd.yaml
kubectl delete namespace bionic-system
```

