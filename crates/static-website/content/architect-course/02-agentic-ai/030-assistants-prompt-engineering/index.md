# Assistants (Prompt Engineering)

Designing a high quality assistant starts with layered prompts. We outline the system, developer, and user prompt shapes we use in production and how to weave guardrails directly into the template.

Examples show how to pass scratchpad context to tools without leaking credentials, plus how to trace prompt revisions via version control.

## Minimal API Example

You can experiment with prompt engineering directly through the same `/api/chat` endpoint used elsewhere in the course. The request below layers the system prompt (“You are a finance assistant”), adds a developer instruction (“Always cite the source”), and passes the user question:

```sh
curl http://localhost:11434/api/chat \
  -H "Content-Type: application/json" \
  -d '{
        "model": "granite4:tiny-h",
        "messages": [
          {"role": "system", "content": "You are a finance assistant that answers clearly and cites sources."},
          {"role": "assistant", "content": "Developer note: cite the latest credit memo if mentioned."},
          {"role": "user", "content": "Summarize last quarter revenue and cite the memo."}
        ]
      }'
```

Each entry in `messages` corresponds to one layer of the prompt stack:

- The **system** message sets tone and guardrails.
- The **assistant** message can carry developer hints or scratchpad state.
- The **user** message is the live input from the analyst.

Tailor the three layers, send the request again, and compare the responses. This tight loop helps data scientists see how even small wording changes affect tool use, citations, and reasoning depth.
