+++
title = "Adding LLM Models"
weight = 95
sort_by = "weight"
+++

K3s will install GPU support if it finds the [NVIDIA container runtime](https://github.com/NVIDIA/libnvidia-container).

## Check for GPU support

Run the following to see if K3s has sucessfully detected Nvidia container support.

```sh
grep nvidia /var/lib/rancher/k3s/agent/etc/containerd/config.toml
```

## Example Adding Microsoft Phi 2

To add a model to your cluster you can create a [Deployment](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/) and a [Service](https://kubernetes.io/docs/concepts/services-networking/service/).

If you apply the following config you should get an instance of [TGI](https://github.com/huggingface/text-generation-inference) running in your cluster. TGI is an inference engine designed to run LLM's in a production setting.

Download the below config into something like `phi-2.yaml` then run

```sh
kubectl apply -f phi-2.yaml
```

## Example Model YAML

```yml
apiVersion: node.k8s.io/v1
kind: RuntimeClass
metadata:
  name: nvidia
handler: nvidia
---
apiVersion: v1
kind: Pod
metadata:
  name: phi-2-gptq
  namespace: bionic-gpt
spec:
  restartPolicy: OnFailure
  runtimeClassName: nvidia
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