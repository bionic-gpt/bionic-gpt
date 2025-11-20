# Connecting LLMs to External Systems

From the sidebar choose `Integrations` and then `Select Integration`.

You'll see an Integration called `Postgres`.

![alt text](select-integration.png)

Click on `Run Integration` to see the Postgres integration up and running.

![alt text](integration-running.png)

## How Tool Calls Enable Integrations

1. **Schema-first contracts**: Each integration exposes a tool definition (OpenAPI-backed) describing the name, JSON schema, and purpose of the action (e.g., `search_salesforce_contacts`, `submit_jira_ticket`). The LLM receives these definitions alongside the user prompt.
2. **Model reasoning**: When the LLM detects that external data is required, it selects the right tool and crafts the arguments (IDs, filters, payloads). Because tools are statically described, the model cannot invent endpoints or parameters.
3. **Execution & safeguards**: The runtime validates the call, enforces authentication scopes, and executes the API request. Responses return to the LLM as structured JSON so the model can cite real values rather than hallucinate.

## Why This Matters

- **_Grounded answers_**: Pull live data—account balances, shipment statuses, policy clauses—so the LLM doesn’t guess.
- **_Closed-loop actions_**: Let assistants open support tickets, schedule meetings, or kick off automations after explaining the plan to the user.
- **_Auditability_**: Every tool call is logged with inputs/outputs, making it easy to review what the assistant did on behalf of the user.
- **_Scoped access_**: Tools respect tenant boundaries and least-privilege permissions; assistants only see what the integration scope allows.

## Example Integrations

- **CRM insights**: A “Customer Health Analyst” assistant uses `list_datasets` for the knowledge base, then calls `search_salesforce_accounts` to enrich the response with up-to-date renewal data.
- **IT automations**: A “Service Desk Copilot” explains troubleshooting steps while calling `create_jira_ticket` and `post_slack_update` to keep humans in the loop.
- **Email triage**: A “Smart Inbox” agent analyzes Gmail threads, calls `classify_email_intent`, and triggers a `send_outreach_sequence` tool when follow-up is required.

By designing integrations as deterministic tool calls, we keep LLM decisions explainable, auditable, and aligned with the systems your organization already trusts.
