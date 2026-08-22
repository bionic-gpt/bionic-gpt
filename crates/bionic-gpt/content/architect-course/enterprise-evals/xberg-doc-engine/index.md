# Vendor Contract Risk Review

This eval tests whether the model can discover the Xberg document-engine
integration, extract a supplied business document, and produce a grounded
procurement risk memo.

The fixture is a deterministic vendor security addendum. The expected answer
must use the extracted document as its source and must distinguish contractual
facts from recommended follow-up questions.

## Test prompt

Download and upload the [vendor security addendum](vendor-security-addendum.md),
then ask:

```text
Use the Xberg Document Engine integration to extract the attached vendor
security addendum. Prepare a concise executive risk memo for procurement.
Identify the obligations, service commitments, liability and termination terms,
and any material gaps that should be resolved before signature. Separate facts
stated in the addendum from your recommendations, and cite the relevant
sections or wording. Do not invent terms that are not in the document.
```

## Expected behavior

1. Discover and call `extractDocument` with the uploaded document.
2. Report the EEA processing restriction and subprocesser approval requirement.
3. Report the 48-hour incident notice obligation.
4. Report the 99.5% availability commitment and 10% service credit.
5. Report the six-month liability cap and its listed exceptions.
6. Report the 30-day material-breach cure period.
7. Flag the missing audit right and post-termination deletion period as gaps,
   not as existing contractual obligations.

## Add the integration

Download the [Xberg OpenAPI specification](xberg-doc-engine.openapi.yaml), add
it under **OpenAPI Specs**, and select it for the integration. The default
development stack already includes the `doc-engine` service.
