# Open AI API

Every inference platform we showcase in the Gen AI Lab—cloud consoles, Docker images, and desktop runtimes—can proxy prompts to the OpenAI API. Keeping that compatibility unlocks a path to GPT-4o, o1, and any future flagship model without changing the rest of the agentic stack.

## Prerequisites

- An OpenAI account with API billing enabled.
- An API key stored as `OPENAI_API_KEY` inside your `.env` or Kubernetes secret.
- Outbound HTTPS access from the machine running Bionic.

## Configuring Bionic

1. Open `configs/model-providers.toml` (or use the UI settings panel) and add an entry that points to `https://api.openai.com/v1`.
2. Select the model you want to expose to assistants, for example `gpt-4o-mini` for low latency tool use or `gpt-4o` for maximum reasoning depth.
3. Toggle **Tool Calling** so the runtime automatically routes structured actions back into Axum handlers.

## Example Call

The snippet below mirrors the structure we use in the labs when testing Ollama. Swap the binary for `curl` and set the `model` field to your preferred OpenAI release:

```sh
curl https://api.openai.com/v1/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
        "model": "gpt-4o-mini",
        "messages": [
          {"role": "system", "content": "You are a helpful assistant."},
          {"role": "user", "content": "List three reasons tool calls matter."}
        ],
        "tool_choice": "auto",
        "tools": [
          {
            "type": "function",
            "function": {
              "name": "log_decision",
              "parameters": {"type": "object", "properties": {"reason": {"type": "string"}}}
            }
          }
        ]
      }'
```

When the assistant emits a tool call, Bionic captures the structured payload exactly like it does for Granite or Ollama, so you can reuse the same handlers and evaluation harnesses.
