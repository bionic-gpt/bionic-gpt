+++
title = "Introduction"
description = "Kubernetes Operator"
weight = 5
sort_by = "weight"
+++

The Kubernetes operator is not yet available

## Bionic Yaml

```yml
# Example of Echo deployment. The operator will receive this specification and will create a deployment of two "echo" pods.
apiVersion: bionic-gpt.com/v1
kind: Bionic # Identifier of the resource type.
metadata:
  name: test-bionic # Name of the "Echo" custom resource instance, may be changed to your liking
  namespace: default # Namespace must exist and account in KUBECONFIG must have sufficient permissions
spec:
  replicas: 1 # Number of "Bionic" pods created.
```

## The Operator

```yml
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: bionic-gpt.com
  namespace: default # For easier deployment and avoid permissions collisions on most clusters, the resource is namespace-scoped. More information at: https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/
spec:
  group: bionic-gpt.com
  names:
    kind: Bionic
    plural: bionics # If not specified, Kubernetes would assume the plural is "Echos"
    singular: bionic
    shortNames:
      - bionic
  scope: Namespaced
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
                  format: int32
              required: ["replicas"]
```