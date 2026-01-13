#!/bin/bash

set -euo pipefail

if [ $# -lt 1 ]; then
    echo "Usage: $0 <version>" >&2
    exit 1
fi

REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "$REPO_ROOT"

# Remove a leading v if there is one
VERSION="$(echo "$1" | sed 's/^v//')"
TAG="v${VERSION}"

DOCKER_DOC="crates/static-website/content/docs/running-locally/docker-compose/index.md"
COMMIT_HASH="$(git log -n 1 --pretty=format:%H -- infra-as-code/docker-compose.yml)"
sed -i "s/[0-9a-f]\{40\}/${COMMIT_HASH}/g" "$DOCKER_DOC"
