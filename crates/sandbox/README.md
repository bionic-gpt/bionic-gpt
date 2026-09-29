# Bionic Sandbox

This crate implements the following interface

```rust
Sandbox::run(RunRequest)
...
pub struct RunRequest {
    pub command: Command,
    pub skills: Vec<SandboxFile>,
    pub openapi_specs: Vec<OpenApiSpec>,
    pub credentials: CredentialSet,
    pub workspace: WorkspaceSnapshot,
    pub tools: Vec<Arc<dyn SandboxTool>>,
}
```

Thats all the information we need to run in the current sandbox which is implemented with [Bashkit](https://bashkit.sh/)

Bashkit is incredible fast, secure and gives us access to `python`, `bash` and even `sqlite`.

Bashkit however doesn't allow for things like installing more tools or python packages.

This can be seen as a good thing as it adds more security.

## Open Sandbox

[Open Sandbox](https://open-sandbox.ai/) is a possible future direction.

This would require the following which is a much more complicated solution than we have at the moment.

- **OpenSandbox control plane** — deploy alongside Bionic; `sandbox-opensandbox` talks to its API.
- **Sandbox lifecycle** — map conversation/session → sandbox; create/reuse/resume/destroy behind `run()`.
- **Sandbox image** — maintain Linux image with Python, bash, curl, git, Typst, LibreOffice, required libraries, etc.
- **Workspace** — expose persistent S3-backed workspace as `/workspace`, probably using FUSE/rclone VFS with local caching/write-back.
- **Skills** — expose supplied skills as `/skills`; via s3?.
- **OpenAPI** — expose supplied specs as `/openapi`; via s3?.
- **Bionic tools** — expose supplied `SandboxTool` callbacks as generated Python/CLI functions. Calls go to an internal endpoint on the existing Bionic server, which invokes the Rust callback. No additional Bionic service/container required.
- **Tool authorization** — sandbox execution gets an opaque identity; Bionic validates which callbacks that execution may invoke.
- **External APIs** — sandbox uses normal HTTP (`curl`, Python, etc.).
- **Credentials** — secrets stay outside the sandbox. OpenSandbox egress/credential injection adds authentication to permitted outbound requests.
- **Egress isolation** — likely an OpenSandbox-managed sidecar container in each sandbox pod.
- **Network policy** — restrict sandbox outbound access to approved destinations.
- **Execution** — run command in `/workspace`; return stdout/stderr/exit code.
- **Later optimization** — warm pools, pause/resume, snapshots, local caches.

Approximate runtime topology:

```text id="zjcbto"
k3s
│
├── Bionic
│     ├── SandboxTool callbacks
│     └── internal tool endpoint
│
├── OpenSandbox control plane
│
└── Sandbox Pod (per active session)
      ├── Linux sandbox
      │     ├── /workspace → S3/cache
      │     ├── /skills
      │     └── /openapi
      │
      └── egress sidecar
             └── credential injection → enterprise APIs
```

The `sandbox-opensandbox` crate contains the adapter logic; OpenSandbox handles Linux/Kubernetes isolation and lifecycle; Bionic continues to own users, permissions, tools, integrations and persistent state.