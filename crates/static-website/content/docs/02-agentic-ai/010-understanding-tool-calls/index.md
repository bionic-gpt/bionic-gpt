# Understanding Tool Calls

Tool calls let the assistant hand off work to real systems so it can fetch information it does not know itself—think “what is the date?” or “look up this account.” That keeps answers accurate, traceable, and aligned with the systems you already trust. This page explains when to create a tool versus a text-only response and how to encode arguments for reliability.

We cover the request lifecycle, how structured responses are marshalled in Rust, and what telemetry you can collect to monitor usage.

## Example: Asking “What is the date?” without Tools

When you omit the `tools` array, the model must answer using its internal context, which usually results in a generic response. The example below uses the same `granite4:tiny-h` model referenced in the Ollama quick start guide:

```sh
curl http://localhost:11434/api/chat \
  -H "Content-Type: application/json" \
  -d '{
        "model": "granite4:tiny-h",
        "messages": [
          {"role": "system", "content": "You are a precise assistant that admits uncertainty."},
          {"role": "user", "content": "What is the date?"}
        ]
      }'
```

Typical output looks like:

```json
{
  "role": "assistant",
  "content": "I do not have access to the current time. Please check your device clock."
}
```

## Example: Asking “What is the date?” with Tools

The `chat` endpoint in Ollama can emit structured tool invocations when the user request requires code. Re-using the `granite4:tiny-h` model, expose a `get_current_datetime` tool so the assistant can call into deterministic logic:

```sh
curl http://localhost:11434/api/chat \
  -H "Content-Type: application/json" \
  -d '{
        "model": "granite4:tiny-h",
        "messages": [
          {"role": "system", "content": "You are a precise assistant that prefers tool calls over guessing."},
          {"role": "user", "content": "What is the date?"}
        ],
        "tools": [
          {
            "type": "function",
            "function": {
              "name": "get_current_datetime",
              "description": "Returns the ISO-8601 timestamp for the current moment.",
              "parameters": {
                "type": "object",
                "properties": {
                  "timezone": {
                    "type": "string",
                    "description": "Optional IANA timezone identifier such as America/New_York."
                  }
                }
              }
            }
          }
        ]
      }'
```

Ollama returns either a natural-language answer or a tool payload. When a tool is selected the `message` looks like:

```json
{
  "role": "assistant",
  "content": "",
  "tool_calls": [
    {
      "name": "get_current_datetime",
      "arguments": "{\"timezone\":\"UTC\"}"
    }
  ]
}
```

The tool payload contains everything your application needs: the tool name and the JSON arguments to pass to your own code. Once you execute the tool and hand the result back to the assistant, it can explain the answer in natural language. Comparing both flows highlights when tools are needed. With a tool definition, the assistant can retrieve the actual date; without one, it can only apologize or guess. Use this contrast when teaching teams why every real-world capability should be backed by deterministic code.
