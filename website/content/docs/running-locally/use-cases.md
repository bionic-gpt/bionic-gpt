+++
title = "How to try out your use cases"
description = "Installing Locally"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 30
sort_by = "weight"

[extra]
toc = true
top = false
+++

We're assuming that you've identified one or more use cases and along the lines of chatting with company private documents.

## Document Upload

Having identified documents that you would the LLM to use as part of it's answers, you can upload those documents via the user interface.

In the background we use a service called unstrutured to remove the text from those documents. So the quality of answers you get is going to depend on the quality of the text within those documents.

![Alt text](/resource-augmented-generation.png "Uploading docements")

## Selecting Datasets

Once you've uploaded your documents go to the chat window and in the prompt dropdown choose 'Default (Use All Datasets)' you can then try out your questions.

![Alt text](/github-readme.png "Uploading docements")

## Collecting Questions

It's probably a good idea to track your questions with an without the datasets to give you some idea of how BionicGPT is working with your data.