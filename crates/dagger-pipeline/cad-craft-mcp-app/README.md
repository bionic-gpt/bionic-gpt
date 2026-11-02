# CADCraft MCP App

The launcher exposes HTTP MCP tools backed by the upstream `cadcraft-cli mcp`
stdio server. Each drawing has its own persistent CLI process. The WASM app
shows a historical, model-generated DXF snapshot after each tool call; it permits
wheel/pinch zoom and middle-button pan, with editing disabled.

Build from the repository root (both CLI and web assets use the resolved commit):

```sh
cargo run -p dagger-pipeline -- cad-craft-mcp-app --cadcraft-ref 258c89fbb021fd22cf25ef519e5caf018b692093
```

The web compatibility patch is checked against upstream source and fails the build
if the expected bridge changes. Use `--cadcraft-ref main` to try newer upstream
versions. The manual GitHub workflow also accepts a ref. `--publish` publishes SHA
and latest tags; otherwise the image is loaded into Docker locally.

## Runtime and registration

Register `http://cad-craft:8080/mcp` in Bionic with **No authentication**.
The service is intended for private, in-cluster access and should not be exposed
through public ingress. Conversation scoping still prevents drawing access across
conversations when requests come through Bionic; any other pod allowed to reach
the service can call it directly and supply conversation metadata.
**Both Bionic and the CadCraft image must be rebuilt** for conversation scoping and
the viewer integration. No public ingress or PUBLIC_BASE_URL is needed.

The container host must support Landlock ABI v3 (Linux 6.2+ with Landlock enabled
and its syscalls permitted by the container runtime). Startup fails if isolation
cannot be enforced; there is no unsandboxed fallback. Workers have empty
environments apart from disabled config persistence, can write only their own
private temporary directory, and can read only that directory, the CLI, runtime
libraries, fonts and null/random devices. Arbitrary upstream file tools are not
advertised. File commands entered through `execute`, scripts or the command line
remain subject to these same filesystem restrictions.

Defaults: port 8080, CLI `/app/cadcraft-cli`, assets `/app/assets`. Local testing
can override `CADCRAFT_LISTEN`, `CADCRAFT_CLI` (absolute executable path), and
`CADCRAFT_ASSETS`.

## Model calls and ownership

1. Discover tools, then call `open_cadcraft {}` to create a drawing.
2. Reuse the returned `structuredContent.drawing_id` in `command_line`, `script`,
   `execute`, `list_commands`, `inspect_drawing`, `query_entities`, `render`, or
   `cancel`. Their remaining parameters and results come from upstream.
3. For example, call `command_line` with
   `{"drawing_id":"<returned ID>","text":"rectang 0,0 100,60"}`, then `execute`
   with `{"drawing_id":"<returned ID>","command":"circle","params":{"center":[10,10],"radius":4}}`.
4. Inspect entities to verify the result. `open_cadcraft {"drawing_id":"…"}`
   reopens an existing drawing without replacing it.

Tools advertise `_meta["bionic/conversationScope"] = true`. Bionic injects
`params._meta["bionic/conversation"]` from authenticated identity and conversation
ID; model arguments cannot supply or override it. The adapter checks this owner
on every drawing access. Browser-initiated tool calls without authenticated
conversation context cannot manipulate drawings. The adapter trusts Bionic to
provide this context; it does not authenticate direct callers, so restrict network
access to the service to trusted in-cluster workloads.

Sessions expire after 24 hours idle or a container restart, with a maximum of 16
active drawings per container. Calls on one drawing serialize. Worker replies are
bounded to 6 MiB with a 60-second deadline. Transport failures stop the session;
commands are never retried automatically because a mutation may already have run.
Expired or failed sessions return an error rather than silently starting blank.
A single adapter replica is required in this version; no distributed state exists.

## Viewer snapshots

Tool results include `_meta["cadcraft/snapshot"]` with version 1, drawing ID,
revision, filename and base64 DXF bytes. Bionic stores the full result for rendering
but excludes app metadata from model context. Snapshot exports are limited to
3 MiB of DXF (4 MiB base64); export failures leave the command result intact and
show a preview error. Earlier conversation results retain their own snapshots,
including after the active drawing session expires. Manual edit synchronization,
durable editable projects and additional 3D capabilities are deferred.

The resource retains the `bionic/assets` manifest and `bionic/assets/read` proxy.
The parent transfers JS/WASM bytes into the opaque-origin iframe, whose loader
accepts tool-result snapshots only from its parent. A patched upstream bridge
then imports those bytes from the loader inside the frame. It has no network
access or same-origin permission. Asset responses retain their 128 MiB ceiling.
The standalone `/` preview opens a blank viewer and loads assets directly.

## Checks

```sh
cargo test --manifest-path crates/dagger-pipeline/cad-craft-mcp-app/Cargo.toml --locked
cargo test -p agent-harness mcp_app_context_tests --locked
node --experimental-vm-modules --test crates/dagger-pipeline/cad-craft-mcp-app/tests/loader.test.cjs
```

For the real CLI smoke test, first build the launcher and upstream CLI, then run:

```sh
python3 crates/dagger-pipeline/cad-craft-mcp-app/tests/smoke.py /absolute/path/to/cad-craft-mcp-app /absolute/path/to/cadcraft-cli
```

This starts a local HTTP adapter and checks real geometry, DXF snapshots, PNG
rendering, concurrent calls, state continuity, conversation ownership,
validation and filesystem isolation. The smoke test uses dummy
web assets; viewer bridge ordering is checked separately by the Node test, and
the patched upstream web app must also compile for `wasm32-unknown-unknown`.
