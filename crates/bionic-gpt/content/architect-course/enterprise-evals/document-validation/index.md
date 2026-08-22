# Vendor Package Validation Against a Procurement Rubric

This eval tests whether the model can validate a real vendor package against a
procurement rubric. It must extract and reconcile a Word agreement, an Excel
service schedule, and a PDF rubric before making a recommendation.

The document-conversion integration is an implementation detail. The user's
goal is to decide whether the vendor package is acceptable, not to learn which
conversion engine processed the files.

## Test inputs

Download and upload all three files:

- [Vendor agreement](vendor-agreement.docx)
- [Vendor service schedule](vendor-service-schedule.xlsx)
- [Procurement security rubric](procurement-security-rubric.pdf)

The PDF is the governing rubric. The Word and Excel files are vendor evidence.

## Test prompt

```text
Validate the uploaded vendor package against the procurement security rubric.

Extract and reconcile the Word agreement and Excel service schedule, then
compare them with every requirement in the PDF rubric. Produce an executive
recommendation for procurement with a requirement-by-requirement table showing
Pass, Fail, Partial, or Unknown, the supporting document and location, and the
action needed for every gap. Do not infer terms that are not stated in the
documents.
```

## Expected behavior

1. Discover and use document extraction for all three formats.
2. Treat the PDF as the authoritative list of requirements.
3. Cite Word clauses, Excel sheets/cells, and PDF rubric requirements.
4. Reconcile evidence across files instead of summarizing each file in
   isolation.
5. Distinguish a failed requirement from an omitted requirement.
6. Classify every requirement as Pass, Fail, Partial, or Unknown.
7. Produce an approve, approve-with-conditions, or do-not-approve
   recommendation supported by the matrix.

## Add the integration

Download the [Document Conversion API OpenAPI specification](document-extraction.openapi.yaml),
add it under **OpenAPI Specs**, and select it for the integration. The default
development stack includes the internal document extraction service.

## Optional reusable skill

The [document-validation skill](document-validation.zip) packages the repeatable
workflow and rubric reference. It can be uploaded from the **Skills** screen
when this validation method should be reusable across conversations.
