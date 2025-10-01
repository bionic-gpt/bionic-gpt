# Install via the Deploy MCP Kubernetes Operator

In environments where you don't want to use the Deploy MCP installer you can install with the Kubernetes operator directly.

## Prerequisites

Deploy MCP requires the following operators to function on your Kubernetes distribution (we regularly test on [K3s](https://k3s.io/), Amazon EKS, and Google Kubernetes Engine): 

[CloudNativePG a Kubernetes operator for Postgres](https://cloudnative-pg.io/)

If you want Ingress (i.e. You're not using Cloudflare Tunnel) then you'll also need the Nginx Ingress Operator

[NGINX and NGINX Plus Ingress Controllers for Kubernetes](https://github.com/nginx/kubernetes-ingress)

### 1. Apply the CustomResourceDefinition (CRD)

```bash
kubectl create namespace deploy-mcp-system
```

And apply the CRD to the cluster (Use the example below)

```bash
kubectl apply -n deploy-mcp-system -f deploy-mcp-crd.yaml
```

### Example CRD YAML (`deploy-mcp-crd.yaml`)

```yaml
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: deploy-mcps.deploy-mcp.com
spec:
  group: deploy-mcp.com
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
                deploy_mcp_db_disk_size:
                  type: integer
                hostname-url:
                  type: string
                hash-deploymcp:
                  type: string
                hash-deploymcp-rag-engine:
                  type: string
                hash-deploymcp-db-migrations:
                  type: string
      subresources:
        status: {}
  scope: Namespaced
  names:
    plural: deploy-mcps
    singular: deploy-mcp
    kind: Deploy MCP
    shortNames:
      - dmc
```

### 2. Install Roles and Service Account

Before deploying the operator, you need to set up the necessary roles and service accounts to grant the operator the required permissions.

Create a `ServiceAccount`, `ClusterRole`, and `ClusterRoleBinding` YAML:

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: deploy-mcp-operator-sa
  namespace: deploy-mcp-system
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: deploy-mcp-operator-role
rules:
  - apiGroups: ["*"]
    resources: ["*"]
    verbs: ["*"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: deploy-mcp-operator-binding
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: deploy-mcp-operator-role
subjects:
  - kind: ServiceAccount
    name: deploy-mcp-operator-sa
    namespace: deploy-mcp-system
```

Apply the roles and service account:

```bash
kubectl apply -n deploy-mcp-system -f deploy-mcp-roles.yaml
```

### 3. Deploy the Operator

You need to deploy the operator (compiled from the Rust code) as a container in your Kubernetes cluster.

Create a `Deployment` YAML for the operator:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: deploy-mcp-operator
  namespace: deploy-mcp-system
spec:
  replicas: 1
  selector:
    matchLabels:
      app: deploy-mcp-operator
  template:
    metadata:
      labels:
        app: deploy-mcp-operator
    spec:
      serviceAccountName: deploy-mcp-operator-sa
      containers:
        - name: deploy-mcp-operator
          image: ghcr.io/deploy-mcp/deploymcp-k8s-operator:1.9.2
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
kubectl apply -n deploy-mcp-system -f deploy-mcp-operator-deployment.yaml
```

### 4. Create a Custom Resource

Now that the operator is running, create an instance of the `Deploy MCP` resource:

First a namespace

```bash
kubectl create namespace deploy-mcp
```

Create a `deploy-mcp-instance.yaml` with the example configuration below.

```yaml
apiVersion: deploy-mcp.com/v1
kind: Deploy MCP
metadata:
  name: deploy-mcp
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
  deploy_mcp_db_disk_size: 20
  hostname-url: "https://example-deploy-mcp.com"
  hash-deploymcp: "abc123"
  hash-deploymcp-rag-engine: "def456"
  hash-deploymcp-db-migrations: "ghi789"
```

Apply this resource:

```bash
kubectl apply -n deploy-mcp -f deploy-mcp-instance.yaml
```

### 5. Verify the Operator is Managing the Resource

Check if the `Deploy MCP` resource has been created:

```bash
kubectl get -n deploy-mcp deploy-mcps
```

Inspect logs of the operator to verify that it's processing the resource:

```bash
kubectl logs -l app=deploy-mcp-operator -n deploy-mcp-system
```

### 6. Clean Up (Optional)

If you want to remove everything:

```bash
kubectl delete -f deploy-mcp-instance.yaml
kubectl delete -f deploy-mcp-operator-deployment.yaml
kubectl delete -f deploy-mcp-roles.yaml
kubectl delete -f deploy-mcp-crd.yaml
kubectl delete namespace deploy-mcp-system
```
