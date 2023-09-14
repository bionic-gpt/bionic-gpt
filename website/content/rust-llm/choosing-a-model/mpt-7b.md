+++
title = "MPT 7B"
description = "T"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 20
sort_by = "weight"


[extra]
toc = true
top = false
section = "rust-llm"
+++

For this guide we'll choose a low parameter model that's been quantized and will therefore run on a developers machine with 8GB or RAM or more. This version of the model doesn't require a GPU.

## MPT-7B

MPT-7B is a decoder-style transformer pre-trained from scratch on 1TB tokens of English text and code. This model was trained by MosaicML.

## To run from the Github container repository

We've already dockerized a quantized model you can try it out.

`docker run -it --rm ghcr.io/purton-tech/mpt-7b-chat`