# Vision Analysis

This eval tests whether the model can use an image-analysis capability as
evidence for a domain-oriented answer. It uses a fixed industrial control-panel
image and a deterministic simulated service, so the evaluation does not call an
external vision model or depend on changing model output.

## Test inputs

- [Download the control-panel image](control-panel.png)
- [Download the image-analysis OpenAPI spec](/architect-course/enterprise-evals/vision-analysis.openapi.yaml)

The spec defines the simulated analysis endpoint. Its response contains
components, visible state, confidence, uncertain observations, and suggested
follow-up checks. It does not contain the final maintenance answer.

## Add the Integration to Bionic

Download the OpenAPI spec, then go to the admin area in Bionic. Open
**OpenAPI Specs**, add a new spec, and paste or upload the vision-analysis YAML.

Return to the app, open **Integrations**, add an integration, and choose the
image-analysis spec. The default lab includes the eval-mocks service used by
the integration.

## Test Prompt

Upload `control-panel.png` to the chat and try:

```text
You are a maintenance engineer. Inspect the uploaded image. Identify the
major components, describe any visible operating state, and flag anything
that may warrant closer inspection. Clearly distinguish observations from
assumptions.
```

## Expected behavior

1. Discover and read the `image-analysis` skill.
2. Identify that the uploaded image requires image analysis.
3. Read the image-analysis function documentation.
4. Call the simulated vision function with the uploaded image reference and
   an appropriate task.
5. Separate observed components and visible state from uncertain conclusions.
6. Produce a concise maintenance-oriented interpretation rather than dumping
   the raw JSON.
7. Recommend follow-up checks without inventing a fault diagnosis.

The answer should not claim an exact gauge reading, a specific controlled
process, pump failure, overpressure, or an electrical fault. Those conclusions
are not supported by the deterministic analysis.

## What to evaluate

The eval passes when the model uses the image, calls the correct function,
preserves uncertainty, and adapts the structured evidence to the maintenance
context. It fails when the model ignores the image, skips the tool, invents
unsupported readings or faults, or simply repeats the response JSON.
