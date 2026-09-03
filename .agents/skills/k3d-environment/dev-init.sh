#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd -- "$script_dir/../../.." && pwd)

cd "$repo_root"

k3d cluster delete k3d-bionic
# 30000: nginx (bionic)
# 30001: postgres (bionic)
# 30002: selenium webdriver
# 30003: selenium vnc
# 30004: mailhog web
# 30005: postgres (selenium)
# 30006: nginx (selenium) So tests can call the api.
k3d cluster create k3d-bionic --agents 1 -p "30000-30006:30000-30006@agent:0"

bash "$script_dir/get-config.sh"
