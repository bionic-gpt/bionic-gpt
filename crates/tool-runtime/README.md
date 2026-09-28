# tool-runtime

This crate provides the tool and integration system used by the app and
`agent-harness`. It exposes OpenAI-style tool definitions, executes tool calls, and
supports both built-in tools and OpenAPI-based external integrations.

## What it does

- Uses rig's `ToolDyn` trait for all executable tools.
- Registers model-facing shell, virtual filesystem, and Python tools.
- Maps execution to provider-neutral operations in the `sandbox` crate.
- Loads system-level and team-connected OpenAPI specs into the function catalogue.
- Executes tool calls and returns JSON results.

## Key modules

- `builtin_tools/` and OpenAPI adapters implement rig `ToolDyn`.
- `tool_catalog.rs`: fixed model-facing built-in tool definitions.
- `tool_dispatcher.rs`: resolve tool instances and execute tool calls.
- `sandbox_dispatcher.rs`: translate model tool calls into sandbox commands.
- `openapi_tool_factory.rs`: OpenAPI v3 parsing and tool definition generation.
- `system_tool_sources.rs`: system-selected OpenAPI specs (per category).
- `builtin_tools/`: built-in tool implementations.
- `tool_auth.rs`: auth token providers for OpenAPI tools.

## Model-facing tools

The model receives fixed definitions for `run_bash`, `read_file`, `write_file`,
`edit_file`, and `run_python`. OpenAPI integrations are discoverable as
markdown files under `/home/user/functions` and are invoked from Python inside
the sandbox; they are not exposed as direct model tools.

## Built-in tools

- `time_date`: get current time and date.
- `web`: open URL tool.
- `run_bash`: sandbox shell tool with `/home/user/skills`, `/home/user/datasets`,
  `/home/user/attachments`, and `rag-search` / `rag-read`.
- `read_file`, `write_file`, `edit_file`: virtual filesystem file operations.
- `run_python`: Monty-backed Python snippets with the virtual filesystem.

## OpenAPI integrations

Prompt integrations are stored in the DB. The flow is:

1. Load prompt integrations and their connections.
2. Parse OpenAPI v3 specs into executable function entries.
3. Build function markdown files and seed them into the sandbox filesystem.
4. Supply executable operations to the sandbox as scoped callbacks.

## Executing tool calls

`execute_tool_calls` accepts a list of OpenAI-style tool calls and dispatches
the fixed built-in tool instances. OpenAPI and Bionic-owned calls are dispatched
through callbacks supplied in `RunRequest.tools`.

## Testing

- `tool_catalog.rs` verifies the fixed model-facing tool definition.
- `tool_dispatcher.rs` includes a tool execution test for the time/date tool.
- `builtin_tools/openapi_tool_adapter.rs` supports HTTP client overrides for tests.
