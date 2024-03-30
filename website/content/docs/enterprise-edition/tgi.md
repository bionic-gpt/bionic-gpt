+++
title = "TGI Inference Engine"
weight = 60
sort_by = "weight"
+++

## The TGI Inference Engine

## Installing a Model with the Operator


```yaml
apiVersion: bionic-gpt.com/v1
kind: Bionic
metadata:
  name: bionic-gpt
  namespace: bionic-gpt 
spec:

  ...
  
  # Single Sign ON
  models:
  - model:
    name: Llama2
    huggingface: TheBloke/Llama-2-13B-GPTQ
    quantization: gptq

...

```