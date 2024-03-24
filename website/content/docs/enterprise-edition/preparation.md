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
docker run --rm --gpus all ubuntu nvidia-smi
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

## Enabling GPU Support in Kubernetes

Install the Nvidia K8s Device Plugin.

```sh
echo '[plugins.cri.containerd.runtimes.runc.options]
  BinaryName = "/usr/bin/nvidia-container-runtime"

[plugins.cri.containerd.runtimes.runc]
  runtime_type = "io.containerd.runc.v2"
[plugins.linux]
  runtime = "nvidia-container-runtime"' > config.toml.tmpl
```

```sh
echo '[plugins."io.containerd.grpc.v1.cri".containerd.runtimes."nvidia"]
  runtime_type = "io.containerd.runc.v2"
[plugins."io.containerd.grpc.v1.cri".containerd.runtimes."nvidia".options]
  BinaryName = "/usr/bin/nvidia-container-runtime"
  SystemdCgroup = true' > config.toml.tmpl
```

```sh
sudo mv config.toml.tmpl /var/lib/rancher/k3s/agent/etc/containerd/config.toml.tmpl
sudo chmod 777 /var/lib/rancher/k3s/agent/etc/containerd/config.toml.tmpl
```

```sh
sudo systemctl restart k3s
```

## Test containerd - How?

```sh

```

## Device Plugin?

```sh
kubectl create -f https://raw.githubusercontent.com/NVIDIA/k8s-device-plugin/v0.14.5/nvidia-device-plugin.yml
```

NVIDIA device plugin should have labelled the node as having an NVIDIA GPU:

```sh
kubectl describe node | grep nvidia.com/gpu
```

You should see some results.

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
