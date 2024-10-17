list:
    just --list

aider:
    aider --no-auto-commits --browser

watch:
    mold -run cargo watch \
        --workdir /workspace/ \
        -w crates/daisy-rsx \
        -w crates/web-pages \
        -w crates/web-server \
        -w crates/db \
        -w crates/web-assets/dist \
        -w crates/web-assets/images \
        --no-gitignore -x "run --bin web-server"

k8s-watch:
    mirrord exec -n bionic-gpt -t deployment/bionic-gpt --steal -- \
        cargo watch \
        --workdir /workspace/ \
        -w crates/daisy-rsx \
        -w crates/web-pages \
        -w crates/web-server \
        -w crates/db \
        -w crates/web-assets/dist \
        -w crates/web-assets/images \
        --no-gitignore -x "run --bin web-server"

open-ports:
    #!/usr/bin/env bash
    kubectl -n bionic-gpt port-forward pod/bionic-db-cluster-1 5433 &
    postgresPID=$!
    kubectl -n bionic-gpt port-forward deployment/mailhog 8025 &
    mailhogPID=$!
    kubectl -n bionic-gpt port-forward deployment/llm-api 11434:11434 &
    ollamaPID=$!
    trap "kill ${ollamaPID} ${mailhogPID} ${postgresPID}; exit 1" INT

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
