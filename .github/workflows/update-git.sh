#!/bin/bash

set -euo pipefail

# Ensure we operate from the repository root regardless of the caller's cwd.
REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "$REPO_ROOT"

# Only stage the files this workflow updates so transient artifacts don't sneak into the commit.
git add crates/k8s-operator/config/bionic.yaml \
        infra-as-code/docker-compose.yml \
        crates/k8s-operator/Cargo.toml

git commit -m "chore(deployment): Update release metadata for $1 [ci skip]"
git push
