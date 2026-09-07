---
name: image-analysis
description: Use image evidence to produce domain-aware answers that distinguish observations and uncertainty.
---
# Image Analysis

Use this skill when the user's task depends on information contained in an uploaded image, photograph, diagram, screenshot, equipment image, or rendered document.

## Workflow

1. Identify the relevant image in `/home/user/attachments`.
2. List `/home/user/functions` and read the relevant image-analysis function documentation before calling it.
3. Send the image reference and a short task describing what needs to be extracted or assessed.
4. Treat the returned analysis as evidence, not as the final answer.
5. Write a useful answer for the user's domain and task.

## Evidence discipline

Separate the answer into directly observed details, uncertain interpretations, and recommended follow-up checks. Never invent text, measurements, labels, equipment states, defects, or diagnoses that are not supported by the analysis.
