#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
cd "$SCRIPT_DIR"

curl -OL https://github.com/dobicinaitis/tailwind-cli-extra/releases/latest/download/tailwindcss-extra-linux-x64
chmod +x tailwindcss-extra-linux-x64
./tailwindcss-extra-linux-x64 -i ./deploy-mcp/input.css -o ./deploy-mcp/dist/tailwind.css --cwd ..

# Generate timestamp and rename tailwind.css file
TIMESTAMP=$(date +%s)
mv ./dist/tailwind.css ./dist/tailwind-${TIMESTAMP}.css

# Update the reference in the source code
sed -i "s|/tailwind\\.css|/tailwind-${TIMESTAMP}.css|g" src/layouts/layout.rs

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"

cargo run
