+++
title = "Adding LLM Models"
weight = 95
sort_by = "weight"
+++

To add a model to your cluster you can create a [Deployment](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/) and a [Service](https://kubernetes.io/docs/concepts/services-networking/service/).

## Example Adding Microsoft Phi 2

If you apply the following config you should get an instance of [TGI](https://github.com/huggingface/text-generation-inference) running in your cluster. TGI is an inference engine designed to run LLM's in a production setting.

```yml
apiVersion: v1
kind: Service
metadata:
  name: phi-2-gptq
  namespace: bionic-gpt
spec:
  selector:
    app: phi-2-gptq
  ports:
    - protocol: TCP
      port: 8000
      targetPort: 80

---

apiVersion: apps/v1
kind: Deployment
metadata:
  name: phi-2-gptq
  namespace: bionic-gpt
spec:
  replicas: 1
  selector:
    matchLabels:
      app: phi-2-gptq
  template:
    metadata:
      labels:
        app: phi-2-gptq
    spec:
      containers:
      - name: tgi
        image: ghcr.io/huggingface/text-generation-inference:1.4
        args: 
          - --model-id 
          - TheBloke/phi-2-GPTQ
          - --quantize 
          - gptq
      resources:
        limits:
          nvidia.com/gpu: 1
```

The model and inference engine must run in the same container.