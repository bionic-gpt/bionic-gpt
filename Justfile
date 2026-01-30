list:
    just --list

dev-init:
    k3d cluster delete k3d-bionic
    # 30000: nginx (bionic)
    # 30001: postgres (bionic)
    # 30002: selenium webdriver
    # 30003: selenium vnc
    # 30004: mailhog web
    # 30005: postgres (selenium)
    # 30006: nginx (selenium) So tests can call the api.
    k3d cluster create k3d-bionic --agents 1 -p "30000-30006:30000-30006@agent:0"
    just get-config

dev-setup:
    stack init
    stack deploy --manifest infra-as-code/stack.yaml --profile dev
    stack deploy --manifest infra-as-code/stack-selenium.yaml

ci:
    cargo run --bin dagger-pipeline -- pull-request

codex: 
    sudo npm install -g @openai/codex

# Retrieve the cluster kube config - so kubectl and k9s work.
get-config:
    sudo apt-get update -qq && sudo apt-get install -y -qq iproute2
    k3d kubeconfig write k3d-bionic --kubeconfig-merge-default
    sed -i "s/127\.0\.0\.1/$(ip route | awk '/default/ {print $3}')/g; s/0\.0\.0\.0/$(ip route | awk '/default/ {print $3}')/g" "$HOME/.kube/config"
    # Disable TLS verification for local dev
    sed -i '/certificate-authority-data/d' "$HOME/.kube/config"
    sed -i '/cluster:/a \ \ \ \ insecure-skip-tls-verify: true' "$HOME/.kube/config"
    echo "✅ kubeconfig updated and TLS verification disabled"

# If you're testing document processing run `just chunking-engine-setup` and `just expose-chunking-engine`
wa env_file=".env":
    #!/usr/bin/env bash
    set -euo pipefail

    if [ ! -f "{{env_file}}" ]; then
        echo "Missing env file: {{env_file}}  run just dot-env" >&2
        exit 1
    fi

    set -a
    . "{{env_file}}"
    set +a

    mold -run cargo watch --workdir /workspace/ \
        -w crates/web-pages -w crates/llm-proxy -w crates/integrations \
        -w crates/web-server -w crates/db -w crates/web-assets/dist \
        -w crates/web-assets/images -w crates/web-assets/typescript \
        -w crates/web-assets/index.ts \
        -w crates/web-assets/input.css \
        --no-gitignore -x "run --bin web-server"

wp:
    npm install --prefix /workspace/crates/web-assets && npm run start --prefix /workspace/crates/web-assets

wt:
    cd /workspace/crates/web-assets && tailwind-extra -i ./input.css -o ./dist/output.css --watch

ws:
    cd /workspace/crates/static-website && cargo watch --workdir /workspace/crates/static-website -w ./content -w ./src --no-gitignore -x "run --bin static-website"

wts:
    cd /workspace/crates/static-website && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch

wds:
    cd /workspace/crates/deploy-mcp && cargo watch --workdir /workspace/crates/deploy-mcp -w ./content -w ./src --no-gitignore -x "run --bin deploy-mcp"

wdts:
    cd /workspace/crates && tailwind-extra -i ./deploy-mcp/input.css -o ./deploy-mcp/dist/tailwind.css --watch

spell:
    docker run --rm -ti -v $HOST_PROJECT_PATH/crates/static-website/content:/workdir tmaier/markdown-spellcheck:latest "**/*.md"

md:
    mirrord exec target/debug/web-server --steal -n bionic-gpt --target deployment/bionic-gpt

test:
    cargo test --workspace --exclude integration-testing --exclude rag-engine

# Look at CONTRIBUTING.md to see how integration testing works
integration-testing test="":
    #!/usr/bin/env bash
    set -euo pipefail

    export DATABASE_URL="postgresql://db-owner:testpassword@host.docker.internal:30005/bionic-gpt?sslmode=disable"
    export WEB_DRIVER_URL="http://host.docker.internal:30002"
    export APPLICATION_URL="http://nginx"
    export MAILHOG_URL="http://host.docker.internal:30004"
    export API_BASE_URL="http://host.docker.internal:30006"

    POD=$(kubectl get pods -n bionic-selenium -l app=selenium -o jsonpath='{.items[0].metadata.name}')
    kubectl exec -n bionic-selenium $POD -- mkdir -p /home/seluser/workspace/files
    kubectl cp crates/integration-testing/files/. bionic-selenium/$POD:/home/seluser/workspace/files

    if [ -n "{{test}}" ]; then
        cargo test -p integration-testing "{{test}}" -- --nocapture
    else
        cargo test -p integration-testing -- --nocapture
    fi

md-selenium:
    cargo build
    mirrord exec target/debug/web-server --steal -n bionic-selenium --target deployment/bionic-gpt

# Install dependencies and optimize architect course screenshots
opt-images:
    sudo apt-get update -qq && sudo apt-get install -y -qq pngquant imagemagick
    # Resize down to max 1200px width (never upscale), strip metadata, then compress with pngquant
    cd crates/static-website/content/architect-course && \
        find . -type f -name '*.png' \
            -print -exec mogrify -resize '1200x>' -strip {} + && \
        find . -type f -name '*.png' \
            -print -exec sh -c 'for f; do pngquant --force --quality 70-85 --ext .png "$f"; done' _ {} +

dot-env:
	#!/usr/bin/env bash
	set -euo pipefail

	cat > .env <<'EOF'
	CHUNKING_ENGINE=http://localhost:8000
	DANGER_JWT_OVERRIDE="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyLCJlbWFpbCI6ImpvaG5AYWNtZS5vcmcifQ.daYgeWqnpmtorlFKjb0sdRFDcPPWfow68KRZh3uUDhc"
	AUTOMATIONS_FEATURE=1
	ENABLE_PROJECTS=1
	LICENCE='{"end_date": "2028-12-31T00:00:00Z", "hostname_url": "http://localhost:7703", "signature": "lMWJJdsUGKepbp7SNCI3Zldl9l0kLOXGbgziBDHk3Q0Jm/ilI4ueDFLx1x/gVmm3xBWHJVCg21OuAm/UlTE5BQ==", "user_count": 2, "app_name": "Bionic", "app_logo_svg": "PHN2ZyB3aWR0aD0iMTQ0IiBoZWlnaHQ9IjE0NCIgdmlld0JveD0iMCAwIDE0NCAxNDQiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPCEtLSBSYWRpYWwgZ3JhZGllbnQgYmFja2dyb3VuZCAtLT4KICA8ZGVmcz4KICAgIDxyYWRpYWxHcmFkaWVudCBpZD0iYmdHcmFkaWVudCIgY3g9IjUwJSIgY3k9IjUwJSIgcj0iNzUlIj4KICAgICAgPHN0b3Agb2Zmc2V0PSIwJSIgc3RvcC1jb2xvcj0iIzRlN2VmZiIvPgogICAgICA8c3RvcCBvZmZzZXQ9IjEwMCUiIHN0b3AtY29sb3I9IiMxZjNjY2YiLz4KICAgIDwvcmFkaWFsR3JhZGllbnQ+CgogICAgPCEtLSBEcm9wIHNoYWRvdyBmaWx0ZXIgLS0+CiAgICA8ZmlsdGVyIGlkPSJkcm9wU2hhZG93IiB4PSItNTAlIiB5PSItNTAlIiB3aWR0aD0iMjAwJSIgaGVpZ2h0PSIyMDAlIj4KICAgICAgPGZlRHJvcFNoYWRvdyBkeD0iMCIgZHk9IjIiIHN0ZERldmlhdGlvbj0iMiIgZmxvb2QtY29sb3I9ImJsYWNrIiBmbG9vZC1vcGFjaXR5PSIwLjciLz4KICAgIDwvZmlsdGVyPgogIDwvZGVmcz4KCiAgPCEtLSBSb3VuZGVkIGJhY2tncm91bmQgLS0+CiAgPHJlY3Qgd2lkdGg9IjE0NCIgaGVpZ2h0PSIxNDQiIHJ4PSIyNCIgcnk9IjI0IiBmaWxsPSJ1cmwoI2JnR3JhZGllbnQpIiAvPgogIDxzdHlsZT4KICAgIC5zbWFsbCB7IAogICAgICAgIGZvbnQ6IG5vcm1hbCAxMjBweCBzYW5zLXNlcmlmOyAKICAgICAgICBmaWxsOiB3aGl0ZTsKICAgIH0KICA8L3N0eWxlPgogIDwhLS0gQmlnZ2VyLCBib2xkZXIgQiB3aXRoIGRyb3Agc2hhZG93IC0tPgogIDx0ZXh0IHg9IjUwJSIgeT0iNTAlIiB0ZXh0LWFuY2hvcj0ibWlkZGxlIiBkb21pbmFudC1iYXNlbGluZT0iY2VudHJhbCIgY2xhc3M9InNtYWxsIiBmaWxsPSJ3aGl0ZSIKICAgICAgICBmaWx0ZXI9InVybCgjZHJvcFNoYWRvdykiPgogICAgQgogIDwvdGV4dD4KPC9zdmc+"}'
	EOF
