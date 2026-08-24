# CLI Gateway

`cli-gateway` exposes a safe, spec-driven HTTP wrapper around fixed CLI
invocations. The OpenAPI document is the source of truth for the HTTP route,
the command arguments, and the CLI binary included in the image.

The Typst specification is at `specs/typst.openapi.yaml`. It accepts repeated
`files` multipart fields, writes them into a temporary workspace, and runs the
fixed `typst compile main.typ output.pdf` command. Callers cannot select the
executable or provide command arguments.

The service exposes `/health`, `/openapi.yaml`, and `/openapi.json` in addition
to the routes declared by the specification.
