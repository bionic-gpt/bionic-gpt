+++
title = "GPU Node Preparation (Nvidia)"
weight = 10
sort_by = "weight"
+++

GPU nodes will need the following installed.

1. [Cuda drivers](https://wiki.debian.org/NvidiaGraphicsDrivers)
1. [Nvidia Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html)
1. Nvidia Runtime Class

## Check Container Toolkit

To see if the container toolkit is correctly installed you can run the following and you should see the connected GPU devices.

```sh
docker run --rm --runtime=nvidia --gpus all ubuntu nvidia-smi
#+-----------------------------------------------------------------------------------------+
#| NVIDIA-SMI 550.54.14              Driver Version: 550.54.14      CUDA Version: 12.4     |
#|-----------------------------------------+------------------------+----------------------+
#| GPU  Name                 Persistence-M | Bus-Id          Disp.A | Volatile Uncorr. ECC |
#| Fan  Temp   Perf          Pwr:Usage/Cap |           Memory-Usage | GPU-Util  Compute M. |
#|                                         |                        |               MIG M. |
#|=========================================+========================+======================|
#|   0  NVIDIA GeForce GTX 1050 Ti     Off |   00000000:1D:00.0  On |                  N/A |
#|  0%   40C    P0             N/A /   90W |    1556MiB /   4096MiB |      0%      Default |
#|                                         |                        |                  N/A |
#+-----------------------------------------+------------------------+----------------------+
#                                                                                         
#+-----------------------------------------------------------------------------------------+
#| Processes:                                                                              |
#|  GPU   GI   CI        PID   Type   Process name                              GPU Memory |
#|        ID   ID                                                               Usage      |
#|=========================================================================================|
#+-----------------------------------------------------------------------------------------+
```

## Install Nvidia Runtime Class

Apply the following YAML

```sh
echo 'apiVersion: node.k8s.io/v1
kind: RuntimeClass
metadata:
  name: nvidia
handler: nvidia' | kubectl apply -f -
```

Then you should see the Nvidia runtime class in your available runtime classes.

```sh
kubectl get runtimeclass
#NAME                  HANDLER               AGE
#crun                  crun                  13h
#lunatic               lunatic               13h
#nvidia                nvidia                13h
#nvidia-experimental   nvidia-experimental   13h
#slight                slight                13h
#spin                  spin                  13h
#wasmedge              wasmedge              13h
#wasmer                wasmer                13h
#wasmtime              wasmtime              13h
#wws                   wws                   13h
```

## Install the Device Plugin

Install helm if you don't have it already.

```sh
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
```

Add the helm repo

```sh
helm repo add nvdp https://nvidia.github.io/k8s-device-plugin
helm repo update
```

```sh
helm upgrade -i nvdp nvdp/nvidia-device-plugin \
  --namespace nvidia-device-plugin \
  --create-namespace \
  --version 0.14.2 \
  --set runtimeClassName=nvidia
```

NVIDIA device plugin should have labelled the node as having an NVIDIA GPU:

```sh
kubectl describe node | grep nvidia.com/gpu
```

You should see some results.

## GPU Discovery (Optional?)

Nvidia [GPU Discovery](https://github.com/NVIDIA/gpu-feature-discovery) gives us a way to see more capabilities on the node.

```sh
helm repo add nvgfd https://nvidia.github.io/gpu-feature-discovery
helm repo update
```

```sh
helm upgrade -i nvgfd nvgfd/gpu-feature-discovery \
  --version 0.8.2 \
  --namespace gpu-feature-discovery \
  --create-namespace \
  --set runtimeClassName=nvidia
```

Now when you run the below you should see more labels on the node.

```sh
kubectl describe node | grep nvidia.com/gpu
```

## Deploy a Test Workload

```sh
echo 'apiVersion: v1
kind: Pod
metadata:
  name: nbody-gpu-benchmark
  namespace: default
spec:
  restartPolicy: OnFailure
  runtimeClassName: nvidia
  containers:
  - name: cuda-container
    image: nvcr.io/nvidia/k8s/cuda-sample:nbody
    args: ["nbody", "-gpu", "-benchmark"]
    resources:
      limits:
        nvidia.com/gpu: 1
    env:
    - name: NVIDIA_VISIBLE_DEVICES
      value: all
    - name: NVIDIA_DRIVER_CAPABILITIES
      value: all' | kubectl apply -f -
```

If everything is working the pod will run and go to state completed.
