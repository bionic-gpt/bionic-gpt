# Agentic Loop

Agentic AI is just **tool calling in a loop** until the model is finished. You are the dispatcher that **runs tools**, returns **results**, and lets the model keep going.


![Alt text](./agent-loop.svg "Agent Loop")

## The Loop (Model ↔ Tools)

1. **Model decides** to call a tool.
2. **Dispatcher executes** the tool with the JSON arguments.
3. **Dispatcher returns** the tool result to the model.
4. **Model continues** with the new information.
5. **Repeat** until the model stops calling tools and returns a final answer.

## Example: You Are the Dispatcher

### 1) Model requests a tool

```json
{
  "role": "assistant",
  "content": "",
  "tool_calls": [
    {
      "name": "get_bitcoin_price_usd",
      "arguments": "{}"
    }
  ]
}
```

### 2) You run the tool and send the result back

```json
{
  "role": "tool",
  "name": "get_bitcoin_price_usd",
  "content": "{\"price_usd\": 68000, \"source\": \"example\"}"
}
```

### 3) Model finishes the response

```json
{
  "role": "assistant",
  "content": "Bitcoin is trading at about $68,000 USD based on the tool result."
}
```

**That’s the whole pattern.** If the model needs more data, it calls another tool. If not, it stops.

## API Call with Full History

Use the full message history when you call the model again after executing a tool.

```sh
curl http://localhost:11434/api/chat \
  -H "Content-Type: application/json" \
  -d '{
    "model": "granite4:tiny-h",
    "stream": false,
    "messages": [
      {
        "role": "system",
        "content": "You are a precise assistant that admits uncertainty."
      },
      {
        "role": "user",
        "content": "What is the price of Bitcoin today in USD?"
      },
      {
        "role": "assistant",
        "content": "",
        "tool_calls": [
          {
            "name": "get_bitcoin_price_usd",
            "arguments": "{}"
          }
        ]
      },
      {
        "role": "tool",
        "name": "get_bitcoin_price_usd",
        "content": "{\"price_usd\": 91337.42 }"
      }
    ],
    "tools": [
      {
        "type": "function",
        "function": {
          "name": "get_bitcoin_price_usd",
          "description": "Returns the current price of Bitcoin in USD.",
          "parameters": {
            "type": "object",
            "properties": {},
            "additionalProperties": false
          }
        }
      }
    ],
    "tool_choice": "auto"
  }'
```
