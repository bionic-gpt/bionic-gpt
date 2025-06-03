# Open Source Models
A wide variety of open-source large language models are available, many of which are refinements of foundation models. Some of these models have been fine-tuned to excel in specific tasks. Below is a curated list of some of the top open-source models currently available.


### General Models

---

#### Lllama3.1 from Meta ####
https://huggingface.co/meta-llama/Meta-Llama-3.1-8B-Instruct

**Model Summary**
The Meta Llama 3.1 collection of multilingual large language models (LLMs) is a collection of pretrained and instruction tuned generative models in 8B, 70B and 405B sizes (text in/text out). The Llama 3.1 instruction tuned text only models (8B, 70B, 405B) are optimized for multilingual dialogue use cases and outperform many of the available open source and closed chat models on common industry benchmarks.


**Intended use**
Llama 3.1 is intended for commercial and research use in multiple languages. Instruction tuned text only models are intended for assistant-like chat, whereas pretrained models can be adapted for a variety of natural language generation tasks. The Llama 3.1 model collection also supports the ability to leverage the outputs of its models to improve other models including synthetic data generation and distillation. The Llama 3.1 Community License allows for these use cases.






### Coding Models

---

#### Granite from IBM ####
https://huggingface.co/ibm-granite/granite-8b-code-instruct-128k

**Model Summary**
Granite-8B-Code-Instruct-128K is a 8B parameter long-context instruct model fine tuned from Granite-8B-Code-Base-128K on a combination of permissively licensed data used in training the original Granite code instruct models, in addition to synthetically generated code instruction datasets tailored for solving long context problems. By exposing the model to both short and long context data, we aim to enhance its long-context capability without sacrificing code generation performance at short input context.

**Intended use**
The model is designed to respond to coding related instructions over long context input up to 128K length and can be used to build coding assistants.



#### Codestral from Mistral ####
https://huggingface.co/mistralai/Mamba-Codestral-7B-v0.1

**Model Summary**
Codestral Mamba is an open code model based on the Mamba2 architecture. It performs on par with state-of-the-art Transformer-based code models.





### Maths

---

#### Mathstral from Mixtral ####
https://huggingface.co/mistralai/Mathstral-7B-v0.1

**Model Summary**
Mathstral 7B is a model specializing in mathematical and scientific tasks, based on Mistral 7B. You can read more in the official [blog post](https://mistral.ai/news/mathstral/).





