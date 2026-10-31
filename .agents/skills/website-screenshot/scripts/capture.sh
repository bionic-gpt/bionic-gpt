#!/usr/bin/env bash
set -euo pipefail
url=${1:?Usage: capture.sh URL [output-directory]}
output_dir=${2:-./tmp}
script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
image=mcr.microsoft.com/playwright/python:v1.55.0-noble
tool_dir=/tmp/bionic-playwright-control
browser_name="website-capture-$$-$(date +%s)"
devcontainer_id=${DEVCONTAINER_ID:-$(hostname)}
port=${BROWSER_DEBUG_PORT:-19223}
printf 'Checking %s\n' "$url"
curl -fsS --max-time 10 -o /dev/null "$url"
docker inspect --type container "$devcontainer_id" >/dev/null
if ! docker image inspect "$image" >/dev/null 2>&1; then
    printf 'Downloading Chromium image\n'
    timeout 120 docker pull "$image"
fi
if ! node -e 'if(require(process.argv[1]).version!=="1.55.0")process.exit(1)' "$tool_dir/package/package.json" >/dev/null 2>&1; then
    printf 'Downloading Playwright control package\n'
    mkdir -p "$tool_dir"
    curl -fLsS --max-time 30 https://registry.npmjs.org/playwright-core/-/playwright-core-1.55.0.tgz -o "$tool_dir/playwright-core.tgz"
    tar -xzf "$tool_dir/playwright-core.tgz" -C "$tool_dir"
fi
node -e 'const s=require("net").createServer();s.on("error",e=>{console.error(e.message);process.exit(1)});s.listen(Number(process.argv[1]),"127.0.0.1",()=>s.close())' "$port"
cleanup() { docker rm -f "$browser_name" >/dev/null 2>&1 || true; }
trap cleanup EXIT
printf 'Starting Chromium\n'
timeout 15 docker run -d --name "$browser_name" --network "container:$devcontainer_id" --entrypoint /ms-playwright/chromium-1187/chrome-linux/chrome "$image" --headless --no-sandbox --disable-dev-shm-usage --no-proxy-server --remote-debugging-address=127.0.0.1 --remote-debugging-port="$port" about:blank >/dev/null
for attempt in {1..20}; do
    if curl -fsS --max-time 1 "http://127.0.0.1:$port/json/version" >/dev/null 2>&1; then break; fi
    sleep 0.25
done
mkdir -p "$output_dir"
output="$output_dir/website-$(date -u +%Y%m%dT%H%M%S%3NZ).png"
node "$script_dir/capture.cjs" "$tool_dir/package" "$url" "$output" "$port"
