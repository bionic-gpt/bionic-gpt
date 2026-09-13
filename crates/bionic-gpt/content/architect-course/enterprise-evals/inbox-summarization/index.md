# Email-to-CRM Automation

This lesson uses the real [AutomationBench](https://github.com/zapier/AutomationBench) task `simple.email_sf_contact_phone_update`. AutomationBench starts with seeded application state, exposes simulated services through ordinary OpenAPI integrations, and evaluates whether the resulting world state is correct.

In this scenario:

- Gmail contains an email from Jordan Lee with a new phone number.
- Salesforce contains Jordan Lee’s existing contact.
- The agent must connect the request across both systems and update the correct CRM record.

## Test Prompt

Use this prompt exactly:

```text
Jordan Lee just emailed us with a new phone number. Can you find that email and update her phone number in Salesforce?
```

## Initialize the task

Before each evaluation, reset the AutomationBench adapter to the named task:

```bash
curl -X POST http://localhost:8880/benchmark/reset \
  -H 'Content-Type: application/json' \
  -d '{"task":"simple.email_sf_contact_phone_update"}'
```

This replaces the adapter’s shared world with the task’s seeded Gmail and Salesforce state. Reset again before each independent run so previous changes cannot affect the result.

## What a good result does

1. Search Gmail for Jordan Lee’s relevant email.
2. Read the email and extract the new phone number.
3. Find Jordan Lee’s contact in Salesforce.
4. Update the correct Salesforce contact.
5. Avoid modifying unrelated records.
6. Confirm the external system state changed instead of merely claiming success.

## How the eval is scored

AutomationBench evaluates the resulting application/world state, not just the assistant’s textual response. For this task, the important final-state assertion is that Jordan Lee’s Salesforce phone number is:

```text
+1-555-0101
```

The assistant’s response can be well written and still fail if Salesforce was not actually updated, or if another contact was changed instead.

## Connect the integrations

Import the AutomationBench OpenAPI integrations and make them available to the team before running the task. The adapter presents Gmail and Salesforce as normal Bionic integrations; no AutomationBench-specific behavior is required in Bionic.
