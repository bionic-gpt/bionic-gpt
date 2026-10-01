# Bionic sandbox

This crate is the provider-neutral execution boundary. It knows how to run a
command, access a caller-supplied virtual filesystem, and issue HTTP requests
through a caller-supplied network mediator:

```rust
pub struct RunRequest {
    pub command: Command,
    pub filesystem: Arc<dyn SandboxFilesystem>,
    pub network: Arc<dyn SandboxNetwork>,
}
```

The crate deliberately has no concepts for conversations, object storage,
datasets, skills, integrations, credentials, OAuth, MCP, or database IDs. Those
belong to the application that constructs the filesystem and network objects.

## Bashkit implementation

`BashkitSandbox` adapts `SandboxFilesystem` to Bashkit's `FileSystem` and
`SandboxNetwork` to Bashkit's `HttpTransport`. Shell commands, `tree`, `ls`,
`cat`, redirects, edits, and Python `open()` therefore operate directly on the
supplied filesystem. `curl` uses the supplied network mediator. No workspace
snapshot is copied into or diffed out of Bashkit.

Bashkit is fast and supports shell, Python, and SQLite, but it does not provide
a general Linux userspace or arbitrary package installation.

## Future providers

A Linux or OpenSandbox adapter should implement the same `Sandbox::run`
contract. The adapter may mount, proxy, or synchronize the filesystem and route
egress through a sidecar, but application authorization and secret resolution
must remain behind `SandboxFilesystem` and `SandboxNetwork`.
