## Running the integration tests

We need to install another Bionic in its own namespace so that the `hostname_url` works inside the cluster.

### Create a Relase Candidate

1. Create a release candidate with the github action
1. Update `stack-selenium.yaml` to use the new version that gets created.

### Local Testing

1. Replace the bionic pod with your local version `just md-selenium`.

### Database 

If you've made changes to the database they'll need to be run into this new namespace.

1. `export DATABASE_URL=postgresql://db-owner:testpassword@host.docker.internal:30005/bionic-gpt?sslmode=disable`
1. `dbmate up`

If you get db issues, you may need to restart the pod.

`psql postgresql://db-owner:testpassword@host.docker.internal:30005/bionic-gpt?sslmode=disable`

### Run the Tests

1. Run the integration tests `just integration-testing`.
1. You can monitor the integration tests via `NoVNC` at `http://localhost:30003` password `secret`.
1. If a test fails, rerun with `RUST_BACKTRACE=1` and include the backtrace in the report.

### Individual Tests

1. `just integration-testing documents`
