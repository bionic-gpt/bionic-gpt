list:
    just --list

dev-init:
    k3d cluster delete
    k3d cluster create --agents 1 -p "30000-30001:30000-30001@agent:0"

dev-setup:
    cargo run --bin k8s-operator -- install --no-operator --testing --development --hostname-url http://localhost:30000
    cargo run --bin k8s-operator -- operator

wa:
    mold -run cargo watch --workdir /workspace/ -w crates/web-pages -w crates/llm-proxy -w crates/integrations -w crates/web-server -w crates/db -w crates/web-assets/dist -w crates/web-assets/images --no-gitignore -x "run --bin web-server"

wp:
    npm install --prefix /workspace/crates/web-assets && npm run start --prefix /workspace/crates/web-assets

wt:
    cd /workspace/crates/web-assets && npx tailwindcss -i ./input.css -o ./dist/output.css --watch

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
    curl -L https://github.com/schemaspy/schemaspy/releases/download/v6.2.4/schemaspy-6.2.4.jar \
        --output tmp/schemaspy.jar
    curl -L https://jdbc.postgresql.org/download/postgresql-42.5.4.jar \
        --output tmp/jdbc-driver.jar

schemaspy:
    java -jar tmp/schemaspy.jar \
        -t pgsql11 \
        -dp tmp/jdbc-driver.jar \
        -db postgres \
        -host db \
        -port 5432 \
        -u postgres \
        -p testpassword \
        -o tmp
    cp -r tmp/diagrams/orphans crates/db/diagrams
    cp -r tmp/diagrams/summary crates/db/diagrams