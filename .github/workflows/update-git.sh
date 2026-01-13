#!/bin/bash

set -euo pipefail

# Ensure we operate from the repository root regardless of the caller's cwd.
REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "$REPO_ROOT"

# Only stage the files this workflow updates so transient artifacts don't sneak into the commit.
git add infra-as-code/docker-compose.yml \
        crates/static-website/content/docs/on-premise/install-linux/index.md \
        crates/static-website/content/docs/running-locally/docker-compose/index.md

git commit -m "chore(deployment): Update release metadata for $1 [ci skip]"
git push
