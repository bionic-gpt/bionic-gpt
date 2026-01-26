## Running the integration tests

We need to install another Bionic in its own namespace so the the `hostname_url` work inside the cluster.

1. Assuming you already have k3d setup. `just dev-selenium` to get an install using http://nginx as the hostname URL.
1. Run `just selenium` to install selenium into `k3d`.
1. Replace the bionic pod with your local version `just md-selenium`.
1. Run the integration tests `cargo test -p integration-testing`.
1. You can monitor the integration tests via `NoVNC` at `http://localhost:7900` password `secret`.