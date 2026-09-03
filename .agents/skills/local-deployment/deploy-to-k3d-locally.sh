#!/usr/bin/env bash
set -euo pipefail

base_image="debian:trixie-slim"
image="bionic-gpt-local:$(date +%s)"
binary="target/debug/web-server"

cluster="k3d-bionic"
expected_context="k3d-k3d-bionic"
namespace="bionic-gpt"
deployment="bionic-gpt"
stackapp="bionic-gpt"

actual_context=$(kubectl config current-context)

if [ "$actual_context" != "$expected_context" ]; then
    echo "Refusing to deploy using Kubernetes context: $actual_context" >&2
    echo "Expected: $expected_context" >&2
    exit 1
fi

# Build using the normal glibc target and existing incremental Cargo cache.
npm run release --prefix crates/web-assets
cargo build --bin web-server

if [ ! -x "$binary" ]; then
    echo "Executable not found: $binary" >&2
    exit 1
fi

# Create a minimal temporary Docker build context.
build_dir=$(mktemp -d)
trap 'rm -rf "$build_dir"' EXIT

mkdir -p \
    "$build_dir/dist" \
    "$build_dir/images"

cp "$binary" "$build_dir/web-server"
cp -a crates/web-assets/dist/. "$build_dir/dist/"
cp -a crates/web-assets/images/. "$build_dir/images/"

cat >"$build_dir/Dockerfile" <<DOCKERFILE
FROM $base_image

RUN apt-get update \
    && apt-get install --no-install-recommends --yes ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY web-server /axum-server
COPY dist/ /workspace/crates/web-assets/dist/
COPY images/ /workspace/crates/web-assets/images/

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

WORKDIR /
USER 1001

ENTRYPOINT ["/axum-server"]
DOCKERFILE

echo "Building $image"

docker build \
    --tag "$image" \
    "$build_dir"

echo "Importing $image into $cluster"

k3d image import "$image" \
    --cluster "$cluster"

echo "Updating $stackapp"

kubectl patch stackapp "$stackapp" \
    --namespace "$namespace" \
    --type merge \
    --patch "{\"spec\":{\"services\":{\"web\":{\"image\":\"$image\"}}}}"

# Wait for the operator to update the generated Deployment before checking
# rollout status. Otherwise rollout status may report success for the old image.
echo "Waiting for the operator to apply $image"

for _ in $(seq 1 60); do
    deployed_image=$(
        kubectl get deployment "$deployment" \
            --namespace "$namespace" \
            --output jsonpath='{.spec.template.spec.containers[0].image}'
    )

    if [ "$deployed_image" = "$image" ]; then
        break
    fi

    sleep 1
done

if [ "${deployed_image:-}" != "$image" ]; then
    echo "Operator did not apply $image" >&2
    echo "Deployment still uses: ${deployed_image:-unknown}" >&2
    exit 1
fi

if ! kubectl rollout status "deployment/$deployment" \
    --namespace "$namespace" \
    --timeout=180s
then
    echo "Rollout failed. Current pods:" >&2
    kubectl get pods --namespace "$namespace" >&2

    echo "Recent application logs:" >&2
    kubectl logs "deployment/$deployment" \
        --namespace "$namespace" \
        --tail=100 >&2 || true

    exit 1
fi

echo
echo "Successfully deployed $image"
echo "Application: http://localhost:30000"
