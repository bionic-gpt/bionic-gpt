# tool-runtime

This crate owns the application side of model tool execution. It constructs a
conversation-scoped virtual filesystem and mediated network, then supplies them
to the provider-neutral `sandbox` crate.

## Responsibilities

- Fixed model-facing tools: `run_bash`, `read_file`, `write_file`, `edit_file`,
  and `run_python`.
- Virtual filesystem routing for skills, datasets, attachments, and persistent
  `/home/user/work` and `/home/user/output` files.
- Lazy loading of remotely backed file content and a write journal for
  persistence.
- Connector discovery from OpenAPI documents.
- Authorization, credential lookup, OAuth refresh, credential injection,
  destination policy, and bounded HTTP execution.
- A future connector extension point for MCP and internal Bionic capabilities.

## Filesystem

The runtime presents application data through `/home/user` without copying it
through `sandbox::RunRequest`. Directory listings and file metadata are
available without loading lazy object bodies. Reading a lazy attachment,
dataset chunk, or persisted file fetches its content on demand. Writes under
`work` and `output` are observed and persisted after execution.

## Connectors

Each authorized integration is exposed as a skill:

```text
/home/user/skills/<connector>/SKILL.md
/home/user/skills/<connector>/openapi.json
```

The OpenAPI document uses an execution-scoped virtual origin such as
`https://gmail.connectors.invalid`. The model inspects the skill/spec and calls
it with ordinary `curl`. `RuntimeNetwork` validates the OpenAPI operation,
maps the virtual origin to the configured upstream, removes caller-supplied
credential headers, injects credentials outside the sandbox, refreshes OAuth
tokens on a 401, and bounds the response. Anonymous public HTTP is a separate
route with no credentials and SSRF checks.

OpenAPI parsing and request-building code remains useful for MCP compatibility,
validation, and metadata extraction. The generated Python-function registry has
been removed.

## Key modules

- `connector_network.rs`: mediated connector, public HTTP, and internal dataset
  routes.
- `lazy_fs.rs`: lazy content and mutation journal over Bashkit's filesystem.
- `sandbox_io.rs`: application filesystem adapter to the neutral sandbox trait.
- `builtin_tools/bashkit.rs`: conversation filesystem construction and output
  persistence.
- `openapi_tool_factory.rs`: OpenAPI parsing and connector metadata.
- `tool_auth.rs`: application-owned token providers and OAuth refresh.
