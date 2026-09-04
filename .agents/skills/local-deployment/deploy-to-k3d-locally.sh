#!/usr/bin/env bash
set -euo pipefail

cluster="k3d-bionic"
expected_context="k3d-k3d-bionic"
namespace="bionic-gpt"
stackapp="bionic-gpt"
rig_log="rig::completions=trace,rig::streaming=trace"
skill_dir=".agents/skills/local-deployment"

usage() {
    echo "Usage: $0 [service ...]"
    echo "Deploy one or more locally built Stack services. Defaults to: web"
    echo "Example: $0 web cli-gateway"
}

if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
    usage
    exit 0
fi

if [ "$#" -eq 0 ]; then
    set -- web
fi

for service in "$@"; do
    if [[ ! "$service" =~ ^[a-z0-9][a-z0-9-]*$ ]]; then
        echo "Invalid service name: $service" >&2
        exit 1
    fi
done

actual_context=$(kubectl config current-context)
if [ "$actual_context" != "$expected_context" ]; then
    echo "Refusing to deploy using Kubernetes context: $actual_context" >&2
    echo "Expected: $expected_context" >&2
    exit 1
fi

build_dir=$(mktemp -d)
trap 'rm -rf "$build_dir"' EXIT
timestamp=$(date +%s)
declare -A deployed_services=()

for service in "$@"; do
    if [ -n "${deployed_services[$service]:-}" ]; then
        continue
    fi

    configured=$(kubectl get stackapp "$stackapp" \
        --namespace "$namespace" \
        --output go-template="{{if index .spec.services \"$service\"}}yes{{end}}")
    if [ "$configured" != "yes" ]; then
        echo "StackApp $stackapp has no service named $service" >&2
        exit 1
    fi

    service_dir="$build_dir/$service"
    image="bionic-gpt-local-$service:$timestamp"
    deployment="$service"
    binary="$service"
    mkdir -p "$service_dir"

    if [ "$service" = "web" ]; then
        binary="web-server"
        deployment="bionic-gpt"
        npm run release --prefix crates/web-assets
        cargo build --bin "$binary"
        cp "target/debug/$binary" "$service_dir/web-server"
        cp -a crates/web-assets/dist "$service_dir/dist"
        cp -a crates/web-assets/images "$service_dir/images"
        docker build \
            --file "$skill_dir/Dockerfile.web" \
            --tag "$image" \
            "$service_dir"
    else
        cargo build --bin "$binary"
        cp "target/debug/$binary" "$service_dir/service"
        target="runtime"
        if [ "$service" = "cli-gateway" ]; then
            target="cli-gateway"
            cp crates/cli-gateway/specs/typst.openapi.yaml "$service_dir/openapi.yaml"
        fi
        docker build \
            --file "$skill_dir/Dockerfile.service" \
            --target "$target" \
            --tag "$image" \
            "$service_dir"
    fi

    k3d image import "$image" --cluster "$cluster"
    kubectl patch stackapp "$stackapp" \
        --namespace "$namespace" \
        --type merge \
        --patch "{\"spec\":{\"services\":{\"$service\":{\"image\":\"$image\"}}}}"

    echo "Waiting for $deployment to use $image"
    deployed_image=""
    for _ in $(seq 1 60); do
        deployed_image=$(kubectl get deployment "$deployment" \
            --namespace "$namespace" \
            --output jsonpath='{.spec.template.spec.containers[0].image}')
        if [ "$deployed_image" = "$image" ]; then
            break
        fi
        sleep 1
    done

    if [ "$deployed_image" != "$image" ]; then
        echo "Deployment $deployment still uses: ${deployed_image:-unknown}" >&2
        exit 1
    fi

    if [ "$service" = "web" ]; then
        kubectl set env "deployment/$deployment" \
            --namespace "$namespace" \
            "RIG_LOG=$rig_log"
    fi

    kubectl rollout status "deployment/$deployment" \
        --namespace "$namespace" \
        --timeout=180s

    deployed_services[$service]="$image"
    echo "Deployed $service as $image"
done

if [ -n "${deployed_services[web]:-}" ]; then
    echo "Application: http://localhost:30000"
fi
