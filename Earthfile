VERSION 0.8
FROM purtontech/rust-on-nails-devcontainer:1.3.1

ARG --global APP_EXE_NAME=web-server
ARG --global OPERATOR_EXE_NAME=k8s-operator
ARG --global RABBITMQ_EXE_NAME=rabbit-mq
ARG --global PIPELINE_EXE_NAME=pipeline-job
ARG --global DBMATE_VERSION=2.2.0

# Folders
ARG --global DB_FOLDER=crates/db
ARG --global PIPELINE_FOLDER=crates/web-assets

# Images with models
ARG --global EMBEDDINGS_IMAGE_NAME=ghcr.io/bionic-gpt/bionicgpt-embeddings-api:cpu-0.6

# This file builds the following containers
ARG --global APP_IMAGE_NAME=bionic-gpt/bionicgpt:latest
ARG --global MIGRATIONS_IMAGE_NAME=bionic-gpt/bionicgpt-db-migrations:latest
ARG --global PIPELINE_IMAGE_NAME=bionic-gpt/bionicgpt-pipeline-job:latest
ARG --global OPERATOR_IMAGE_NAME=bionic-gpt/bionicgpt-k8s-operator:latest
ARG --global RABBITMQ_IMAGE_NAME=bionic-gpt/bionicgpt-rabbitmq:latest

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
    BUILD +pipeline-job-container
    BUILD +rabbitmq-container

all:
    BUILD +migration-container
    BUILD +app-container
    BUILD +operator-container
    BUILD +pipeline-job-container
    BUILD +rabbitmq-container

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
    COPY --dir crates/daisy-rsx crates/daisy-rsx
    RUN cd $PIPELINE_FOLDER && npm run release
    SAVE ARTIFACT $PIPELINE_FOLDER/dist

pipeline-job-container:
    FROM scratch
    COPY +build/$PIPELINE_EXE_NAME pipeline-job
    ENTRYPOINT ["./pipeline-job"]
    SAVE IMAGE --push $PIPELINE_IMAGE_NAME
     

rabbitmq-container:
    FROM scratch
    COPY +build/$RABBITMQ_EXE_NAME rabbit-mq
    ENTRYPOINT ["./rabbit-mq"]
    SAVE IMAGE --push $RABBITMQ_IMAGE_NAME

build-web-server:
    # Copy in all our crates
    COPY --dir crates crates
    RUN rm -rf crates/rabbit-mq crates/k8s-operator crates/pipeline-job
    COPY --dir Cargo.lock Cargo.toml .
    COPY --dir +npm-build/dist $PIPELINE_FOLDER/

    # We need to run inside docker as we need postgres running for cornucopia
    ARG DATABASE_URL=postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable
    USER root
    WITH DOCKER \
        --pull ankane/pgvector
        RUN docker run -d --rm --network=host -e POSTGRES_PASSWORD=testpassword ankane/pgvector \
            && dbmate --wait --migrations-dir $DB_FOLDER/migrations up \
            && cargo leptos build --release -vv
    END
    SAVE ARTIFACT target/release/$APP_EXE_NAME
    SAVE ARTIFACT target/site


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
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$PIPELINE_EXE_NAME
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$RABBITMQ_EXE_NAME
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
    COPY +build/$APP_EXE_NAME axum-server
    # Place assets in a build folder as that's where statics is expecting them.
    COPY --dir +npm-build/dist /build/$PIPELINE_FOLDER/
    COPY --dir $PIPELINE_FOLDER/images /build/$PIPELINE_FOLDER/images
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
    FROM joseluisq/rust-linux-darwin-builder:1.76.0
    COPY --dir crates/k8s-operator .
    RUN cd k8s-operator \ 
        && CC=o64-clang \
        CXX=o64-clang++ \
        cargo build --release --target x86_64-apple-darwin
    SAVE ARTIFACT k8s-operator/target/x86_64-apple-darwin/release/k8s-operator AS LOCAL ./bionic-cli-darwin

build-cli-windows:
    RUN sudo apt update && sudo apt upgrade -y 
    RUN sudo apt install -y g++-mingw-w64-x86-64 
    
    RUN rustup target add x86_64-pc-windows-gnu 
    RUN rustup toolchain install stable-x86_64-pc-windows-gnu 

    COPY --dir crates/k8s-operator .
    RUN cd k8s-operator \ 
        && cargo build --release --target x86_64-pc-windows-gnu
    SAVE ARTIFACT k8s-operator/target/x86_64-pc-windows-gnu/release/k8s-operator.exe AS LOCAL ./bionic-cli-windows.exe

# AWS Deployment
bionic-cluster-delete:
    ARG AWS_ACCESS_KEY_ID
    ARG AWS_SECRET_ACCESS_KEY
    RUN curl -sLO "https://github.com/eksctl-io/eksctl/releases/latest/download/eksctl_Linux_amd64.tar.gz" \
        && tar -xzf eksctl_Linux_amd64.tar.gz -C /tmp && rm eksctl_Linux_amd64.tar.gz \
        && sudo mv /tmp/eksctl /usr/local/bin
    RUN eksctl delete cluster -n bionic-gpt -r us-east-2

bionic-cluster-update:
    ARG AWS_ACCESS_KEY_ID
    ARG AWS_SECRET_ACCESS_KEY
    ARG AWS_ACCOUNT_ID
    RUN sudo apt-get update && sudo apt-get install -y awscli
    RUN curl -sLO "https://github.com/eksctl-io/eksctl/releases/latest/download/eksctl_Linux_amd64.tar.gz" \
        && tar -xzf eksctl_Linux_amd64.tar.gz -C /tmp && rm eksctl_Linux_amd64.tar.gz \
        && sudo mv /tmp/eksctl /usr/local/bin
    RUN curl -sLO "https://github.com/bionic-gpt/bionic-gpt/releases/latest/download/bionic-cli-linux" \
        && sudo mv ./bionic-cli-linux /usr/local/bin/bionic \
        && sudo chmod +x /usr/local/bin/bionic
    RUN bionic -V
    RUN eksctl utils write-kubeconfig --cluster bionic-gpt --region us-east-2
    RUN kubectl get nodes
    RUN bionic install --pgadmin --hostname-url https://app.bionic-gpt.com

bionic-cluster-create:
    ARG AWS_ACCESS_KEY_ID
    ARG AWS_SECRET_ACCESS_KEY
    ARG AWS_ACCOUNT_ID
    ARG TUNNEL_TOKEN
    RUN sudo apt-get update && sudo apt-get install -y awscli
    COPY --dir infra-as-code .
    RUN curl -sLO "https://github.com/eksctl-io/eksctl/releases/latest/download/eksctl_Linux_amd64.tar.gz" \
        && tar -xzf eksctl_Linux_amd64.tar.gz -C /tmp && rm eksctl_Linux_amd64.tar.gz \
        && sudo mv /tmp/eksctl /usr/local/bin
    RUN curl -sLO "https://github.com/bionic-gpt/bionic-gpt/releases/latest/download/bionic-cli-linux" \
        && sudo mv ./bionic-cli-linux /usr/local/bin/bionic \
        && sudo chmod +x /usr/local/bin/bionic
    RUN bionic -V
    RUN sed -i "s/{ACCOUNT_ID}/$AWS_ACCOUNT_ID/g" ./infra-as-code/cluster.yaml
    RUN cat ./infra-as-code/cluster.yaml
    RUN eksctl create cluster -f ./infra-as-code/cluster.yaml
    RUN kubectl get nodes
    RUN bionic install --pgadmin --hostname-url https://app.bionic-gpt.com
    RUN kubectl -n bionic-gpt create secret generic cloudflare-credentials --from-literal=token=$TUNNEL_TOKEN
    RUN kubectl -n bionic-gpt apply -f ./infra-as-code/cloudflare.yaml

# One docker container with all our services
community-edition:
    FROM purtontech/rust-on-nails-devcontainer:1.3.1
    COPY +build/$APP_EXE_NAME /usr/bin/web-server
    COPY +build/$PIPELINE_EXE_NAME /usr/bin/pipeline-job
    ENTRYPOINT ["/usr/bin/web-server"]