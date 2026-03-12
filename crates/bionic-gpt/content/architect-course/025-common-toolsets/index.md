# Common Toolsets

An **agent runtime** is the operating environment around the model.
The model reasons, but the runtime gives it **state**, **tools**, **execution**, and **time**.

Diagram of the [Open Claw](https://openclaw.ai) runtime tools.

![Alt text](./agent-runtime.svg "Common Toolsets")

## Runtime Capabilities

- **Memory**: recall prior facts and context.
- **Sandbox**: safely read, write, edit, and execute code.
- **Cron**: run jobs on a schedule.
- **Skills**: packaged workflows and constraints.
- **Toolsets (OpenAPI, web, etc.)**: connect external systems.

Without a runtime, you have a chat model.
With a runtime, you have an **agent** that can act reliably over time.
