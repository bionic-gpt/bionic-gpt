## Running the integration tests

We need to install another Bionic in its own namespace so that the `hostname_url` works inside the cluster.

### Create a Relase Candidate

1. Create a release candidate with the github action
1. Update `stack-selenium.yaml` to use the new version that gets created.

### Install

1. Assuming you already have k3d setup. `just dev-selenium` to get an install using http://nginx as the hostname URL.
1. Run `just selenium` to install selenium into `k3d`.
1. Replace the bionic pod with your local version `just md-selenium`.

### Database Migrations?

If you've made changes to the database they'll need to be run into this new namespace.

1. `kubectl port-forward pod/bionic-gpt-db-cluster-1 5432:5432 -n bionic-selenium`
1. `DATABASE_URL=postgresql://db-owner:testpassword@localhost:5432/bionic-gpt?sslmode=disable dbmate status`

### Run the Tests

1. Run the integration tests `cargo test -p integration-testing`.
1. You can monitor the integration tests via `NoVNC` at `http://localhost:7900` password `secret`.