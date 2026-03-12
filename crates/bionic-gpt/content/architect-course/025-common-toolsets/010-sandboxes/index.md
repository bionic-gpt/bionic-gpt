# Sandboxes

Outline of potential Tool Defintions for a Sandbox.

```js
# Read a file from the sandbox
read(path: string): string

# Write a file in the sandbox
write(path: string, content: string): string

# Apply a patch/edit to an existing file
edit(path: string, diff: string): string

# Execute code and return stdout/stderr
exec(code: string): string

# Run a command/process in the sandbox
process(command: string, args: string[]): string
```

## Sandboxes (What & Why)

A **sandbox** is an isolated execution environment where an LLM can safely run code or tools without access to the host system.

LLMs use sandboxes to:

* Execute code securely
* Prevent data leaks or system damage
* Enforce resource limits (CPU, memory, time)
* Run untrusted or user-generated instructions

In agentic systems, sandboxes enable **safe autonomy**:

> *The model can act, experiment, and fail — without breaking production.*

Common sandbox examples:

* Python execution environments
* Containerized tool runners
* WASM-based runtimes

**No sandbox → no safe tool execution → no real agent behavior.**


## Further Reading

- [OpenClaw Sandbox](https://docs.openclaw.ai/gateway/sandboxing)
- [Code Mode: give agents an entire API in 1,000 tokens](https://blog.cloudflare.com/code-mode-mcp/)
- [IronClaw sandbox implementation](https://github.com/nearai/ironclaw/tree/main/src/sandbox)
- [AI Sandboxes Startup](https://e2b.dev/)
- [Just Bash](https://github.com/vercel-labs/just-bash) Simulates a bash environment with configurable tools.
