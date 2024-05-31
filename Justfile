list:
    just --list

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
    sed -i "s/bionic_docker_compose = \".*\"/bionic_docker_compose = \"$COMMIT_HASH\"/" website/config.toml

release:
    #!/usr/bin/env bash
    export LATEST_TAG=$(git describe --tags --abbrev=0)
    echo $LATEST_TAG    
    sed -i "s/bionic_version = \".*\"/bionic_version = \"$LATEST_TAG\"/" website/config.toml


