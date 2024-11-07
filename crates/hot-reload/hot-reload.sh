#!/bin/bash
WATCH_FILE="/workspace/target/debug/web-server"

run_commands() {
    echo "Change detected in ${WATCH_FILE}. Running commands..."
    start_time=$(date +%s)
    POD_NAME=$(kubectl get pods -n bionic-gpt -l app=bionic-gpt -o jsonpath="{.items[0].metadata.name}")
    kubectl cp /workspace/target/debug/web-server bionic-gpt/$POD_NAME:/app/new-server
    kubectl exec -it --namespace=bionic-gpt $POD_NAME -- bash -c "rm -rf /workspace/crates/web-assets/dist"
    kubectl exec -it --namespace=bionic-gpt $POD_NAME -- bash -c "rm -rf /workspace/crates/web-assets/images"
    kubectl cp /workspace/crates/web-assets/dist/ bionic-gpt/$POD_NAME:/workspace/crates/web-assets/
    kubectl cp /workspace/crates/web-assets/images bionic-gpt/$POD_NAME:/workspace/crates/web-assets/
    kubectl cp /workspace/hot-reload/new-server.txt bionic-gpt/$POD_NAME:/app/new-server.txt
    end_time=$(date +%s)
    duration=$((end_time - start_time))
    echo "Deployment completed in ${duration} seconds."
}

echo "Watching file: ${WATCH_FILE}"

while true; do
    inotifywait -e modify "${WATCH_FILE}"
    run_commands
done