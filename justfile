help:
    just -h

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
    kubectl -n bionic-gpt port-forward pod/bionic-db-cluster-1 5433 &
    postgresPID=$!
    kubectl -n bionic-gpt port-forward deployment/mailhog 8025 &
    mailhogPID=$!
    kubectl -n bionic-gpt port-forward 0.0.0.0 deployment/llm-api 11434:11434 &
    ollamaPID=$!
    trap "kill ${ollamaPID} ${mailhogPID} ${postgresPID}; exit 1" INT

k8s-db:
    export DATABASE_URL=$(kubectl get secret database-urls -n bionic-gpt -o jsonpath="{.data.migrations-url}" | base64 --decode | sed "s/bionic-db-cluster-rw/localhost/; s/\?sslmode=require//")
    psql $DATABASE_URL