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
