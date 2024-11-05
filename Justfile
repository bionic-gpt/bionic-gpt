list:
    just --list

aider:
    aider --no-auto-commits --browser


hot-reload:
    export RESTART_DATE=$(date +%s)
    echo $RESTART_DATE

    # Create the Dockerfile
    echo 'FROM debian:12-slim' > Dockerfile
    echo 'COPY target/debug/web-server axum-server' >> Dockerfile
    echo 'COPY crates/web-assets/dist /workspace/crates/web-assets/dist/' >> Dockerfile
    echo 'COPY crates/web-assets/images /workspace/crates/web-assets/images' >> Dockerfile
    echo 'RUN chmod +x ./axum-server' >> Dockerfile
    echo 'ENTRYPOINT ["./axum-server"]' >> Dockerfile

    # Build, import, and patch
    docker build -t "ghcr.io/bionic-gpt/bionic-gpt:${RESTART_DATE}" .
    k3d image import "ghcr.io/bionic-gpt/bionic-gpt:${RESTART_DATE}"
    kubectl patch deployment bionic-gpt -n bionic-gpt -p \
    "{\"spec\": {\"template\": {\"spec\": {\"containers\": [{\"name\": \"bionic-gpt\", \"image\": \"ghcr.io/bionic-gpt/bionic-gpt:${RESTART_DATE}\", \"imagePullPolicy\": \"Never\"}]}}}}"

    # Clean up by deleting the Dockerfile
    rm Dockerfile
    # Record the end time and calculate the duration
    export END_TIME=$(date +%s)
    export DURATION=$((END_TIME - RESTART_DATE))
    @echo "Hot reload completed in ${DURATION} seconds."

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