+++
title = "Installation for GPU"
description = "Installation for GPU"
weight = 10
sort_by = "weight"
+++

## Configure to run with Text Generation Interface and litellm

The standard bionicGPT install comes with a CPU based llama2-7B model installed but we can access many more models using a combination of TGI and litellm and this includes using GPU for better inference timings. This setup allows you local access to the hundreds of thousands of models listed on HuggingFace. 

![Alt text](../arch.png "Architecture")

\

### 1. Install and run TGI Docker Container
Full details of this container can be found at [HuggingFace TGI](https://github.com/huggingface/text-generation-inference)


```sh
docker run --gpus all --shm-size 1g -p 8080:80 -v ./models:/data ghcr.io/huggingface/text-generation-inference:1.2 --model-id TheBloke/zephyr-7B-beta-AWQ --max-batch-prefill-tokens 2048 --quantize awq
```
This downloads the TheBloke/zephyr-7B-beta-AWQ model with inference using GPU. If you haven't run Docker containers with GPU install required libraries from  [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html) for other GPU setup refer to [TGI](https://github.com/huggingface/text-generation-inference)



To test that this is running go to [localhost:8080/docs](http://localhost:8080/docs)
![Alt text](../TGI-web.png "TGI Web Interface")

\
\
### 2. Install and run litellm Docker Container

```sh
docker run -p 6789:6789 ghcr.io/berriai/litellm:main-v1.10.3  --model huggingface/TheBloke/zephyr-7B-beta-AWQ --api_base http://127.0.0.1:8080/generate_stream --host 0.0.0.0 --port 6789
```

To test that this is running go to [localhost:6789/docs](http://localhost:6789)
![Alt text](../litellm-web.png "litellm Web Interface")


Use the v1/models tab to get the models installed. This should return the model you installed under TGI
\
![Alt text](../model.png "model installed under TGI")
 

\
\
### 3. Configure bionicGPT to communicate with litellm

It is important that for the URL to litellm you use your machine's IP address and not localhost as these are running in different Docker spaces. E.g. http://192/168.86.40:6789/v1

Go to the Model Setup screen and click Add Model

![Alt text](../bionic-setup.png "bionicGPT to litellm setup")


### 4. Create a prompt associated with the new model

\
\
\
### 5. Select the new prompt for inference