# CADCraft MCP App

Build from the repository root:

```sh
cargo run -p dagger-pipeline -- cad-craft-mcp-app --cadcraft-ref main
```

The manual GitHub workflow accepts an upstream ref. `--publish` publishes SHA
and latest tags. Without it the image is loaded into Docker locally.

The independent Rust launcher listens on 8080. Register
`http://cad-craft:8080/mcp` with Bionic inside the cluster. No PUBLIC_BASE_URL
or public ingress is required. Both Bionic and this image must be rebuilt.

The resource includes a `bionic/assets` file manifest. Its inline loader requests
files with `bionic/assets/read`; Bionic authorizes each request and proxies files
from the registered server's `/assets/` endpoint. The parent transfers bytes to
the opaque-origin iframe, where blob JavaScript and explicit WASM bytes start
the editor. The iframe has no network access or same-origin permission.

The standalone `/` preview loads assets directly. This POC has no shared drawing
storage or agent-driven drawing commands. Asset responses are buffered with a
128 MiB ceiling; streaming to the browser is deferred.
