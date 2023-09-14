+++
title = "Introduction"
description = "LLM Ops"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 10
sort_by = "weight"

[extra]
toc = true
top = false
section = "rust-llm"
+++

When creating software it's often a good practice to document the architecture using  a technique called [Architecture Decision Records](https://adr.github.io/).

An ADR is nothing more than a markdown document that records the title, status, context, decision, and consequences of a particular design choice.

When a decision is made it's often helpful to create a small Proof of Concept that illustrates how the decision will play out in the real world. 

I found previously when working on the [Rust on Nails](https://rust-on-nails.com/) architecture that when you combine the ADR's with Proofs of Concept the result reads like a tutorial on how to build a system in a particular way. 

## What are we going to build?

BionicGPT is an architecture/tutorial on how to build an enterprise AI chat system that gives answers to questions based on documents held within the company.

Further, the aim is to setup a development process that allows the developer to get going with the hardware they already have. 

## Mockup of the User Interface

![UI](/github-readme.png)

## Architecture Diagram

N.B. This will change.

![LLM Operations](../llm-ops.svg)