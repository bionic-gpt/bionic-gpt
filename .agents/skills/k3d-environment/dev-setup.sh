#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd -- "$script_dir/../../.." && pwd)

cd "$repo_root"

stack init --install-keycloak
stack deploy --manifest infra-as-code/stack.yaml --profile dev
stack deploy --manifest infra-as-code/stack-selenium.yaml
