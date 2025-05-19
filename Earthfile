VERSION 0.8

FROM purtontech/rust-on-nails-devcontainer:1.3.17

ARG --global APP_EXE_NAME=web-server
ARG --global OPERATOR_EXE_NAME=k8s-operator
ARG --global AIRBYTE_EXE_NAME=airbyte-connector
ARG --global RAG_ENGINE_EXE_NAME=rag-engine
ARG --global DBMATE_VERSION=2.2.0

# Folders
ARG --global DB_FOLDER=crates/db
ARG --global PIPELINE_FOLDER=crates/web-assets

# Images with models
ARG --global EMBEDDINGS_IMAGE_NAME=ghcr.io/bionic-gpt/bionicgpt-embeddings-api:cpu-0.6

# This file builds the following containers
ARG --global APP_IMAGE_NAME=bionic-gpt/bionicgpt:latest
ARG --global CE_IMAGE_NAME=bionic-gpt/bionic-ce:latest
ARG --global MIGRATIONS_IMAGE_NAME=bionic-gpt/bionicgpt-db-migrations:latest
ARG --global RAG_ENGINE_IMAGE_NAME=bionic-gpt/bionicgpt-rag-engine:latest
ARG --global OPERATOR_IMAGE_NAME=bionic-gpt/bionicgpt-k8s-operator:latest
ARG --global AIRBYTE_IMAGE_NAME=bionic-gpt/bionicgpt-airbyte-connector:latest
ARG --global HOT_RELOAD_IMAGE_NAME=bionic-gpt/bionicgpt-hot-reload:latest

WORKDIR /build

USER vscode

dev:
    BUILD +pull-request
    # On github this check is performed directly by the action
    #BUILD +integration-test
    #BUILD +check-selenium-failure

pull-request:
    BUILD +migration-container
    BUILD +app-container
    BUILD +operator-container
    BUILD +rag-engine-container
    BUILD +airbyte-connector-container

all:
    BUILD +migration-container
    BUILD +app-container
    BUILD +operator-container
    BUILD +rag-engine-container
    BUILD +airbyte-connector-container

npm-deps:
    COPY $PIPELINE_FOLDER/package.json $PIPELINE_FOLDER/package.json
    COPY $PIPELINE_FOLDER/package-lock.json $PIPELINE_FOLDER/package-lock.json
    COPY --dir $PIPELINE_FOLDER/patches $PIPELINE_FOLDER/patches
    RUN cd $PIPELINE_FOLDER && npm install
    SAVE ARTIFACT $PIPELINE_FOLDER/node_modules

npm-build:
    FROM +npm-deps
    COPY $PIPELINE_FOLDER $PIPELINE_FOLDER
    COPY +npm-deps/node_modules $PIPELINE_FOLDER/node_modules
    COPY --dir crates/web-pages crates/web-pages
    RUN cd $PIPELINE_FOLDER && npm run release
    SAVE ARTIFACT $PIPELINE_FOLDER/dist

rag-engine-container:
    FROM scratch
    # Don't run as root 
    USER 1001
    COPY --chown=1001:1001 +build/$RAG_ENGINE_EXE_NAME rag-engine
    ENTRYPOINT ["./rag-engine"]
    SAVE IMAGE --push $RAG_ENGINE_IMAGE_NAME
     

airbyte-connector-container:
    FROM scratch
    # Don't run as root 
    USER 1001
    COPY --chown=1001:1001 +build/$AIRBYTE_EXE_NAME airbyte-connector
    ENTRYPOINT ["./airbyte-connector"]
    SAVE IMAGE --push $AIRBYTE_IMAGE_NAME

build:
    # Copy in all our crates
    COPY --dir crates crates
    COPY --dir Cargo.lock Cargo.toml .
    COPY --dir +npm-build/dist $PIPELINE_FOLDER/
    # We need to run inside docker as we need postgres running for cornucopia
    ARG DATABASE_URL=postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable
    USER root
    WITH DOCKER \
        --pull ankane/pgvector
        RUN docker run -d --rm --network=host -e POSTGRES_PASSWORD=testpassword ankane/pgvector \
            && dbmate --wait --migrations-dir $DB_FOLDER/migrations up \
            && cargo build --release --target x86_64-unknown-linux-musl
    END
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$APP_EXE_NAME
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$RAG_ENGINE_EXE_NAME
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$AIRBYTE_EXE_NAME
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$OPERATOR_EXE_NAME

migration-container:
    FROM alpine
    RUN apk add --no-cache \
        curl \
        postgresql-client \
        tzdata
    RUN curl -OL https://github.com/amacneil/dbmate/releases/download/v$DBMATE_VERSION/dbmate-linux-amd64 \
        && mv ./dbmate-linux-amd64 /usr/bin/dbmate \
        && chmod +x /usr/bin/dbmate
    COPY --dir $DB_FOLDER .
    CMD dbmate up
    SAVE IMAGE --push $MIGRATIONS_IMAGE_NAME

# To test this locally run
# docker run -it --rm -e APP_DATABASE_URL=$APP_DATABASE_URL -p 7703:7703 bionic-gpt/bionicgpt:latest
app-container:
    FROM scratch
    # Don't run as root 
    USER 1001
    COPY --chown=1001:1001 +build/$APP_EXE_NAME axum-server
    # Place assets in a build folder as that's where statics is expecting them.
    COPY --dir --chown=1001:1001 +npm-build/dist /build/$PIPELINE_FOLDER/
    COPY --dir --chown=1001:1001 $PIPELINE_FOLDER/images /build/$PIPELINE_FOLDER/images
    ENTRYPOINT ["./axum-server"]
    SAVE IMAGE --push $APP_IMAGE_NAME

# We've got a Kubernetes operator
operator-container:
    FROM scratch
    COPY +build/$OPERATOR_EXE_NAME k8s-operator
    ENTRYPOINT ["./k8s-operator", "operator"]
    SAVE IMAGE --push $OPERATOR_IMAGE_NAME

# Embeddings container - download models from huggungface
embeddings-container-base:
    FROM purtontech/rust-on-nails-devcontainer:1.1.17
    RUN sudo apt install -y python3-venv python3-pip
    RUN sudo pip install -U "huggingface_hub[cli]" --break-system-packages
    RUN sudo huggingface-cli download --cache-dir ./data BAAI/bge-small-en-v1.5 1_Pooling/config.json
    RUN sudo huggingface-cli download --cache-dir ./data BAAI/bge-small-en-v1.5 model.safetensors
    RUN sudo huggingface-cli download --cache-dir ./data BAAI/bge-small-en-v1.5 config.json
    RUN sudo huggingface-cli download --cache-dir ./data BAAI/bge-small-en-v1.5 tokenizer.json
    SAVE ARTIFACT ./data
embeddings-container:
    FROM ghcr.io/huggingface/text-embeddings-inference:cpu-0.6
    COPY +embeddings-container-base/data /data
    CMD ["--json-output", "--model-id", "BAAI/bge-small-en-v1.5"]
    SAVE IMAGE --push $EMBEDDINGS_IMAGE_NAME

build-cli-linux:
    COPY --dir crates/k8s-operator .
    RUN cd k8s-operator && cargo build --release
    SAVE ARTIFACT k8s-operator/target/release/k8s-operator AS LOCAL ./bionic-cli-linux

build-cli-osx:
    FROM joseluisq/rust-linux-darwin-builder:1.84.1
    COPY --dir crates/k8s-operator .
    RUN cd k8s-operator && CC=o64-clang CXX=o64-clang++ cargo build --release --target x86_64-apple-darwin
    SAVE ARTIFACT k8s-operator/target/x86_64-apple-darwin/release/k8s-operator AS LOCAL ./bionic-cli-darwin

build-cli-windows:
    RUN sudo apt update && sudo apt upgrade -y 
    RUN sudo apt install -y g++-mingw-w64-x86-64 
    
    RUN rustup target add x86_64-pc-windows-gnu

    COPY --dir crates/k8s-operator .
    RUN cd k8s-operator && cargo build --release --target x86_64-pc-windows-gnu
    SAVE ARTIFACT k8s-operator/target/x86_64-pc-windows-gnu/release/k8s-operator.exe AS LOCAL ./bionic-cli-windows.exe

# docker run -p 8000:8000 bionic-gpt/openapi-time:latest
openapi-time:
    FROM mcp/time:latest

    RUN pip install mcpo uv

    # Default command to run the Python proxy
    ENTRYPOINT ["uvx", "mcpo", "--host", "0.0.0.0", "--port", "8000", "--", "mcp-server-time","--local-timezone=America/New_York"]

    SAVE IMAGE --push bionic-gpt/openapi-time:latest

# docker run -p 8000:8000 --add-host=host.docker.internal:host-gateway bionic-gpt/openapi-postgres:latest postgresql://db-owner:testpassword@host.docker.internal:30001/bionic-gpt?sslmode=disable
# curl localhost:8000/openapi.json
# curl -X POST http://localhost:8000/query -H 'Content-Type: application/json' -d '{"sql": "SELECT * FROM users"}'

openapi-postgres:
    FROM mcp/postgres:latest

    # install python3 and the venv tool
    RUN apk add --no-cache python3 py3-virtualenv

    # make and activate a venv, install your packages there
    RUN python3 -m venv /opt/venv \
     && /opt/venv/bin/pip install --upgrade pip mcpo uv

    # ensure our venv's pip/python are on PATH
    ENV PATH="/opt/venv/bin:${PATH}"

    # now uvx & mcpo will be picked up from that venv
    ENTRYPOINT ["uvx", "mcpo", \
                "--host", "0.0.0.0", \
                "--port", "8000", \
                "--", "node", "dist/index.js"]

    SAVE IMAGE --push bionic-gpt/openapi-postgres:latest