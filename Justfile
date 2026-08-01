set dotenv-load := true

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
    stack init --install-keycloak
    stack deploy --manifest infra-as-code/stack.yaml --profile dev
    stack deploy --manifest infra-as-code/stack-selenium.yaml

ci:
    cargo run --bin dagger-pipeline -- pull-request

codex:
    sudo apt update && sudo apt install -y bubblewrap
    sudo chmod u+s /usr/bin/bwrap
    sudo chown -R vscode:vscode /home/vscode/.codex
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
wa:
    mold -run cargo watch --workdir /workspace/ \
        -w crates/web-pages -w crates/agent-runtime -w crates/tool-runtime \
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
    cd /workspace/crates/bionic-gpt && cargo watch --workdir /workspace/crates/bionic-gpt -w ./content -w ./src --no-gitignore -x "run --bin bionic-gpt"

wts:
    cd /workspace/crates/bionic-gpt && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch

spell:
    docker run --rm -ti -v $HOST_PROJECT_PATH/crates/bionic-gpt/content:/workdir tmaier/markdown-spellcheck:latest "**/*.md"

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

dev:
    @if [ ! -f .env ]; then just dot-env; fi
    cargo binstall --no-confirm zellij
    zellij -l .devcontainer/layout.kdl

website:
    cargo binstall --no-confirm zellij
    zellij -l .devcontainer/layout-site.kdl

stop:
    #!/usr/bin/env bash
    set -euo pipefail

    zellij delete-all-sessions --force --yes || true

    project_dir="{{ justfile_directory() }}"
    process_groups=()

    while read -r pid; do
        [ -n "$pid" ] || continue
        process_dir=$(readlink "/proc/$pid/cwd" 2>/dev/null || true)
        if [ "$process_dir" = "$project_dir" ]; then
            process_groups+=("$pid")
            kill -TERM -- "-$pid" 2>/dev/null || true
        fi
    done < <(pgrep -f '^just (wa|wp|wt|ws|wts)$' || true)

    for _ in {1..20}; do
        any_running=false
        for process_group in "${process_groups[@]}"; do
            if kill -0 -- "-$process_group" 2>/dev/null; then
                any_running=true
                break
            fi
        done
        [ "$any_running" = true ] || break
        sleep 0.25
    done

    for process_group in "${process_groups[@]}"; do
        kill -KILL -- "-$process_group" 2>/dev/null || true
    done

dot-env:
	#!/usr/bin/env bash
	set -euo pipefail

	cat > .env <<'EOF'
	CHUNKING_ENGINE=http://localhost:8000
	DANGER_JWT_OVERRIDE="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyLCJlbWFpbCI6ImpvaG5AYWNtZS5vcmcifQ.daYgeWqnpmtorlFKjb0sdRFDcPPWfow68KRZh3uUDhc"
	EOF
