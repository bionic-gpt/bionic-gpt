# From Chatbot to AI Computer

In [Understanding Tool Calls](/architect-course/020-basics-of-tool-calls/010-understanding-tool-calls/), we asked a model:

> What is the price of Bitcoin today in USD?

The model could not answer from its training data because the price changes constantly. We solved that problem by giving it a purpose-built tool named `get_bitcoin_price_usd`.

That tool adds one useful capability. If we want the model to retrieve an exchange rate, inspect a CSV, resize an image, or generate a report, we could keep adding purpose-built tools. Alternatively, we can give the model a working environment containing software that already knows how to perform many of those tasks.

This is the shift from chatbot to **AI computer**.

An AI computer is a conversational workspace in which a model can inspect and create files, execute commands or code, use installed tools and skills, maintain working context, connect to external systems, schedule future work, and produce finished artifacts.

Here, “AI computer” describes the software environment around the model. It does not mean AI-specific hardware, a GPU workstation, or a consumer “AI PC.”

## Give the Model a Computer

Let us ask the Bitcoin question again. This time we will not provide a Bitcoin-specific tool. We will provide one general tool that can execute a Bash command inside an isolated environment:

```json
{
  "type": "function",
  "function": {
    "name": "exec_bash",
    "description": "Execute a Bash command in an isolated workspace and return stdout and stderr.",
    "parameters": {
      "type": "object",
      "properties": {
        "command": {
          "type": "string",
          "description": "The Bash command to execute."
        }
      },
      "required": ["command"],
      "additionalProperties": false
    }
  }
}
```

The user still asks:

```txt
What is the price of Bitcoin today in USD?
```

The model knows that it needs current information. It also knows that `curl` can make an HTTP request, so it can use the general execution tool to call a public price API:

```json
{
  "role": "assistant",
  "content": "",
  "tool_calls": [
    {
      "name": "exec_bash",
      "arguments": {
        "command": "curl --fail --silent --show-error https://api.coinbase.com/v2/prices/BTC-USD/spot"
      }
    }
  ]
}
```

The model does not execute this command itself. The application receives the tool call, runs the command inside the permitted sandbox, and returns stdout and stderr.

A successful command returns JSON similar to this:

```json
{
  "data": {
    "amount": "64251.42",
    "base": "BTC",
    "currency": "USD"
  }
}
```

The amount above is illustrative. Bitcoin prices change continuously.

The application adds the command result to the conversation, and the model can now answer:

```txt
Bitcoin is currently trading at approximately $64,251.42 USD.
```

The agentic loop is the same as before: the model requests a tool, the application executes it, and the result is returned to the model. What changed is the breadth of the tool.

## From One Capability to Many

`get_bitcoin_price_usd` encodes one operation chosen by a developer. `exec_bash` exposes a controlled environment containing existing software. The model can compose that software to perform tasks that were not each implemented as separate tools.

| Purpose-built tool | AI computer |
| --- | --- |
| The developer defines the operation | The model selects and combines permitted operations |
| `get_bitcoin_price_usd` retrieves one price | `curl` can call many permitted HTTP endpoints |
| The result is returned directly | Results can be inspected, transformed, saved, and reused |
| A new use case often needs a new tool | Existing software can support many related use cases |

For example, the same environment could let the model:

1. Retrieve prices for several currencies with `curl`.
2. Save the responses as working files.
3. Use a script to compare the values.
4. Generate a chart or report.
5. Return the finished file as an artifact.

The user asks for an outcome. The model decides which permitted software to use, checks intermediate results, and continues until it has produced that outcome. Many workflows therefore no longer require a separately coded agent application to coordinate every step. The conversational environment can provide much of that orchestration.

This does not mean that one Bash tool provides unlimited access or makes every task reliable. Its capabilities depend on the commands, files, credentials, network destinations, and other resources made available by the surrounding environment.

## The Parts of an AI Computer

The model provides reasoning and control, while the environment provides somewhere for that reasoning to act.

| Component | Role |
| --- | --- |
| Chat interface | User interface |
| Model | Reasoning and control |
| Sandbox/runtime | Compute environment |
| Files | Working storage |
| Skills and tools | Software |
| Integrations | Network and enterprise access |
| Memory/projects | Persistent context |
| Scheduled tasks | Scheduler |
| Artifacts | Outputs |

Together, these components let the model inspect uploaded files, create intermediate results, run tools, preserve useful context, connect to other systems, and return a document, spreadsheet, image, or other finished artifact.

## A Computer Needs Boundaries

A general execution tool is more powerful than a purpose-built price tool, so it also creates a larger security boundary. Commands and model-generated code should be treated as untrusted.

A production environment needs controls around:

* filesystem access;
* permitted network destinations;
* installed commands and system calls;
* CPU, memory, process count, and execution time;
* credentials and access to host services;
* logging, approvals, and audit history.

The model should receive enough access to complete the task, but no more.

## The Sandbox Is One Component

A sandbox gives the model an isolated place to execute commands and manipulate files. In the Bitcoin example, it is where the application safely runs `curl`.

The sandbox is the compute environment inside the AI computer, not the whole computer. The complete working environment also includes files, skills, memory, integrations, scheduling, and artifacts. The following lessons examine those parts and the boundaries between them.
