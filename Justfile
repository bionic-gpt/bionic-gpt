list:
    just --list

dev-init:
    k3d cluster delete
    k3d cluster create --agents 1 -p "30000-30001:30000-30001@agent:0"

dev-setup:
    cargo run --bin k8s-operator -- install --no-operator --testing --development --hostname-url http://localhost:30000
    cargo run --bin k8s-operator -- operator

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
    k3d kubeconfig write k3s-default --kubeconfig-merge-default

# Good for feeding the schema into the AI.
dump-schema:
    pg_dump --schema-only --no-owner --no-privileges --file=schema.sql $DATABASE_URL


# If you're testing document processing run `just chunking-engine-setup` and `just expose-chunking-engine`
wa:
    CHUNKING_ENGINE=http://localhost:8000 \
    AUTOMATIONS_FEATURE=1 \
    LICENCE='{"end_date":"2028-12-31T00:00:00Z","hostname_url":"http://localhost:7703","signature":"lMWJJdsUGKepbp7SNCI3Zldl9l0kLOXGbgziBDHk3Q0Jm/ilI4ueDFLx1x/gVmm3xBWHJVCg21OuAm/UlTE5BQ==","user_count":2}' \
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