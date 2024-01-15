+++
title = "Adding LLM Models"
weight = 95
sort_by = "weight"
+++

To add a model to your cluster you can create a [Deployment](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/).

## Example Adding Mixtral 8x7B

This section is a work in progress but deploying a new model will look something like this.

```yml
apiVersion: v1
kind: Service
metadata:
  name: 8x7b-service
spec:
  selector:
    app: 8x7b
  ports:
    - protocol: TCP
      port: 80
      targetPort: 80

---

apiVersion: apps/v1
kind: Deployment
metadata:
  name: 8x7b-deployment
spec:
  replicas: 3  # Number of desired replicas
  selector:
    matchLabels:
      app: 8x7b
  template:
    metadata:
      labels:
        app: 8x7b
    spec:
      containers:
      - name: 8x7b
        image: your-8x7b-container-image:tag
        ports:
        - containerPort: 80

```