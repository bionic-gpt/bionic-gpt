list:
    just --list

dev-init:
    k3d cluster delete k3d-bionic
    k3d cluster create k3d-bionic --agents 1 -p "30000-30001:30000-30001@agent:0"

dev-setup:
    cargo run --bin k8s-operator -- install --no-operator --testing --development --hostname-url http://localhost:30000
    cargo run --bin k8s-operator -- operator

ci:
    cargo run --bin dagger-pipeline -- pull-request

ci-all:
    cargo run --bin dagger-pipeline -- pull-request

codex: 
    sudo npm install -g @openai/codex

# Upgrade the testing chunking engine to the real one
chunking-engine-setup:
    kubectl set image deployment/chunking-engine \
        chunking-engine=downloads.unstructured.io/unstructured-io/unstructured-api:4ffd8bc \
        -n bionic-gpt
    kubectl patch deployment chunking-engine -n bionic-gpt \
        --type='json' \
        -p='[{"op": "remove", "path": "/spec/template/spec/containers/0/command"}]'

expose-chunking-engine:
    kubectl -n bionic-gpt port-forward --address 0.0.0.0 deployment/chunking-engine 8000:8000

# Retrieve the cluster kube config - so kubectl and k9s work.
get-config:
    sudo apt-get update -qq && sudo apt-get install -y -qq iproute2
    k3d kubeconfig write k3d-bionic --kubeconfig-merge-default
    sed -i "s/127\.0\.0\.1/$(ip route | awk '/default/ {print $3}')/g; s/0\.0\.0\.0/$(ip route | awk '/default/ {print $3}')/g" "$HOME/.kube/config"
    # Disable TLS verification for local dev
    sed -i '/certificate-authority-data/d' "$HOME/.kube/config"
    sed -i '/cluster:/a \ \ \ \ insecure-skip-tls-verify: true' "$HOME/.kube/config"
    echo "✅ kubeconfig updated and TLS verification disabled"

# Good for feeding the schema into the AI.
dump-schema:
    pg_dump --schema-only --no-owner --no-privileges --file=schema.sql $DATABASE_URL


# If you're testing document processing run `just chunking-engine-setup` and `just expose-chunking-engine`
wa:
    CHUNKING_ENGINE=http://localhost:8000 \
    AUTOMATIONS_FEATURE=1 \
    LICENCE='{"end_date": "2028-12-31T00:00:00Z", "hostname_url": "http://localhost:7703", "signature": "lMWJJdsUGKepbp7SNCI3Zldl9l0kLOXGbgziBDHk3Q0Jm/ilI4ueDFLx1x/gVmm3xBWHJVCg21OuAm/UlTE5BQ==", "user_count": 2, "app_name": "Bionic", "app_logo_svg": "PHN2ZyB3aWR0aD0iMTQ0IiBoZWlnaHQ9IjE0NCIgdmlld0JveD0iMCAwIDE0NCAxNDQiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPCEtLSBSYWRpYWwgZ3JhZGllbnQgYmFja2dyb3VuZCAtLT4KICA8ZGVmcz4KICAgIDxyYWRpYWxHcmFkaWVudCBpZD0iYmdHcmFkaWVudCIgY3g9IjUwJSIgY3k9IjUwJSIgcj0iNzUlIj4KICAgICAgPHN0b3Agb2Zmc2V0PSIwJSIgc3RvcC1jb2xvcj0iIzRlN2VmZiIvPgogICAgICA8c3RvcCBvZmZzZXQ9IjEwMCUiIHN0b3AtY29sb3I9IiMxZjNjY2YiLz4KICAgIDwvcmFkaWFsR3JhZGllbnQ+CgogICAgPCEtLSBEcm9wIHNoYWRvdyBmaWx0ZXIgLS0+CiAgICA8ZmlsdGVyIGlkPSJkcm9wU2hhZG93IiB4PSItNTAlIiB5PSItNTAlIiB3aWR0aD0iMjAwJSIgaGVpZ2h0PSIyMDAlIj4KICAgICAgPGZlRHJvcFNoYWRvdyBkeD0iMCIgZHk9IjIiIHN0ZERldmlhdGlvbj0iMiIgZmxvb2QtY29sb3I9ImJsYWNrIiBmbG9vZC1vcGFjaXR5PSIwLjciLz4KICAgIDwvZmlsdGVyPgogIDwvZGVmcz4KCiAgPCEtLSBSb3VuZGVkIGJhY2tncm91bmQgLS0+CiAgPHJlY3Qgd2lkdGg9IjE0NCIgaGVpZ2h0PSIxNDQiIHJ4PSIyNCIgcnk9IjI0IiBmaWxsPSJ1cmwoI2JnR3JhZGllbnQpIiAvPgogIDxzdHlsZT4KICAgIC5zbWFsbCB7IAogICAgICAgIGZvbnQ6IG5vcm1hbCAxMjBweCBzYW5zLXNlcmlmOyAKICAgICAgICBmaWxsOiB3aGl0ZTsKICAgIH0KICA8L3N0eWxlPgogIDwhLS0gQmlnZ2VyLCBib2xkZXIgQiB3aXRoIGRyb3Agc2hhZG93IC0tPgogIDx0ZXh0IHg9IjUwJSIgeT0iNTAlIiB0ZXh0LWFuY2hvcj0ibWlkZGxlIiBkb21pbmFudC1iYXNlbGluZT0iY2VudHJhbCIgY2xhc3M9InNtYWxsIiBmaWxsPSJ3aGl0ZSIKICAgICAgICBmaWx0ZXI9InVybCgjZHJvcFNoYWRvdykiPgogICAgQgogIDwvdGV4dD4KPC9zdmc+"}' \
    mold -run cargo watch --workdir /workspace/ \
        -w crates/web-pages -w crates/llm-proxy -w crates/integrations \
        -w crates/web-server -w crates/db -w crates/web-assets/dist \
        -w crates/web-assets/images -w crates/web-assets/typescript \
        -w crates/web-assets/scss -w crates/web-assets/index.ts \
        -w crates/web-assets/input.css \
        --no-gitignore -x "run --bin web-server"

wad:
    CHUNKING_ENGINE=http://localhost:8000 \
    LICENCE='{"default_lang":"en-US","end_date":"2028-12-31T00:00:00Z","redirect_url":"/app/team/{team_id}/integrations","hostname_url":"http://localhost:7703","user_count":100000,"app_name":"Deploy","app_logo_svg":"PHN2ZyBmaWxsPSIjMDAwMDAwIiB2aWV3Qm94PSIwIDAgMzIgMzIiIGlkPSJpY29uIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgo8ZyBpZD0iU1ZHUmVwb19iZ0NhcnJpZXIiIHN0cm9rZS13aWR0aD0iMCI+PC9nPgo8ZyBpZD0iU1ZHUmVwb190cmFjZXJDYXJyaWVyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiPjwvZz4KPGcgaWQ9IlNWR1JlcG9faWNvbkNhcnJpZXIiPgo8ZGVmcz48c3R5bGU+LmNscy0xe2ZpbGw6bm9uZTt9PC9zdHlsZT48L2RlZnM+Cjx0aXRsZT5kZXBsb3ktcnVsZXM8L3RpdGxlPgo8cG9seWdvbiBwb2ludHM9IjE4IDQgMTIgMTAgMTMuNDEgMTEuNDEgMTcgNy44MyAxNyAyMCAxOSAyMCAxOSA3LjgzIDIyLjU5IDExLjQxIDI0IDEwIDE4IDQiPjwvcG9seWdvbj4KPHJlY3QgeD0iOCIgeT0iMTgiIHdpZHRoPSI3IiBoZWlnaHQ9IjIiPjwvcmVjdD4KPHJlY3QgeD0iOCIgeT0iMjIiIHdpZHRoPSIxNiIgaGVpZ2h0PSIyIj48L3JlY3Q+CjxyZWN0IHg9IjgiIHk9IjI2IiB3aWR0aD0iMTYiIGhlaWdodD0iMiI+PC9yZWN0Pgo8cmVjdCBpZD0iX1RyYW5zcGFyZW50X1JlY3RhbmdsZV8iIGRhdGEtbmFtZT0iJmx0O1RyYW5zcGFyZW50IFJlY3RhbmdsZSZndDsiIGNsYXNzPSJjbHMtMSIgd2lkdGg9IjMyIiBoZWlnaHQ9IjMyIj48L3JlY3Q+CjwvZz4KPC9zdmc+","signature":"gQfV2HeWBNW25ZY1SMrhKjN3u4sfEeg82v+wpKGl8xYHM5PekiBi7XWvD4AL6wuumKfxg7V15+NJHEuO5grFBg=="}' \
    mold -run cargo watch --workdir /workspace/ \
        -w crates/web-pages -w crates/llm-proxy -w crates/integrations \
        -w crates/web-server -w crates/db -w crates/web-assets/dist \
        -w crates/web-assets/images -w crates/web-assets/typescript \
        -w crates/web-assets/scss -w crates/web-assets/index.ts \
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
    docker run --rm -ti -v /workspace/crates/static-website/content:/workdir tmaier/markdown-spellcheck:latest "**/*.md"

md:
    mirrord exec target/debug/web-server --steal -n bionic-gpt --target deployment/bionic-gpt

test:
    cargo test --workspace --exclude integration-testing --exclude rag-engine

# Look at CONTRIBUTING.md to see how integration testing works
integration-testing:
    export WEB_DRIVER_URL=http://localhost:4444 && \
    export APPLICATION_URL=http://nginx-development && \
    cargo test --workspace --exclude rag-engine

# Similar to dev setup, but so that selenium works
testing-setup:
    cargo run --bin k8s-operator -- install --no-operator --testing --development --hostname-url http://nginx-development
    cargo run --bin k8s-operator -- operator

# Install Selenium in the bionic-gpt namespace
selenium:
    printf '%s\n' \
        'apiVersion: v1' \
        'kind: Pod' \
        'metadata:' \
        '  name: selenium-chrome' \
        '  namespace: bionic-gpt' \
        '  labels:' \
        '    app: selenium-chrome' \
        'spec:' \
        '  containers:' \
        '  - name: chrome' \
        '    image: selenium/standalone-chrome' \
        '    ports:' \
        '    - containerPort: 4444' \
        '    volumeMounts:' \
        '    - name: dshm' \
        '      mountPath: /dev/shm' \
        '  # Mirrors --shm-size=2g from .devcontainer/docker-compose.yml and CI' \
        '  volumes:' \
        '  - name: dshm' \
        '    emptyDir:' \
        '      medium: Memory' \
        '      sizeLimit: 2Gi' \
    | kubectl replace --force -f -
    kubectl wait --for=condition=Ready pod/selenium-chrome -n bionic-gpt --timeout=60s
    kubectl port-forward pod/selenium-chrome 4444:4444 7900:7900 -n bionic-gpt

release-docker:
    #!/usr/bin/env bash
    export COMMIT_HASH=$(git log -n 1 --pretty=format:"%H" -- infra-as-code/docker-compose.yml)
    echo $COMMIT_HASH
    sed -i "s/[0-9a-f]\{40\}/$COMMIT_HASH/g" crates/static-website/content/docs/running-locally/docker-compose/index.md

release:
    #!/usr/bin/env bash
    export LATEST_TAG=$(git describe --tags --abbrev=0)
    echo $LATEST_TAG    
    sed -i "s/export BIONIC_VERSION=.*/export BIONIC_VERSION=$LATEST_TAG/" crates/static-website/content/docs/on-premise/install-linux/index.md

schemaspy-install:
    sudo apt update
    sudo apt install default-jre graphviz
    mkdir -p tmp
    curl -L https://github.com/schemaspy/schemaspy/releases/download/v6.2.4/schemaspy-6.2.4.jar \
        --output tmp/schemaspy.jar
    curl -L https://jdbc.postgresql.org/download/postgresql-42.5.4.jar \
        --output tmp/jdbc-driver.jar

schemaspy:
    java -jar tmp/schemaspy.jar \
        -t pgsql11 \
        -dp tmp/jdbc-driver.jar \
        -db bionic-gpt \
        -host localhost \
        -port 30001 \
        -u db-owner \
        -p testpassword \
        -o tmp
    cp -r tmp/diagrams/orphans/orphans.png crates/db/diagrams
    cp -r tmp/diagrams/summary/relationships.real.large.png crates/db/diagrams