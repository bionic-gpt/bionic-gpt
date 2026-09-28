# sandbox

Provider-neutral code execution for Bionic agents.

The public API is a single asynchronous `Sandbox::run(RunRequest)` operation.
Requests contain a command, skill files, OpenAPI metadata, resolved credentials,
a workspace snapshot, and authorization-scoped callback tools. Implementations
own filesystem synchronization and lifecycle details.

`BashkitSandbox` is the initial implementation. This crate must not depend on
Bionic's database, object storage, agent harness, or model-facing tool runtime.

