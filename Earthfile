VERSION 0.7
FROM purtontech/rust-on-nails-devcontainer:1.1.17

ARG --global APP_EXE_NAME=axum-server
ARG --global OPERATOR_EXE_NAME=k8s-operator
ARG --global PIPELINE_EXE_NAME=pipeline-job
ARG --global DBMATE_VERSION=2.2.0

# Folders
ARG --global AXUM_FOLDER=crates/axum-server
ARG --global DB_FOLDER=crates/db
ARG --global PIPELINE_FOLDER=crates/asset-pipeline

# Base images
ARG --global ENVOY_PROXY=envoyproxy/envoy:v1.28.0
ARG --global KEYCLOAK_BASE_IMAGE=quay.io/keycloak/keycloak:23.0

# This file builds the following containers
ARG --global APP_IMAGE_NAME=bionic-gpt/bionicgpt:latest
ARG --global ENVOY_IMAGE_NAME=bionic-gpt/bionicgpt-envoy:latest
ARG --global KEYCLOAK_IMAGE_NAME=bionic-gpt/bionicgpt-keycloak:latest
ARG --global MIGRATIONS_IMAGE_NAME=bionic-gpt/bionicgpt-db-migrations:latest
ARG --global PIPELINE_IMAGE_NAME=bionic-gpt/bionicgpt-pipeline-job:latest
ARG --global TESTING_IMAGE_NAME=bionic-gpt/bionicgpt-integration-tests:latest
ARG --global OPERATOR_IMAGE_NAME=bionic-gpt/bionicgpt-k8s-operator:latest

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
    BUILD +testing-container
    BUILD +operator-container
    BUILD +envoy-container
    BUILD +keycloak-container
    BUILD +pipeline-job-container

all:
    BUILD +migration-container
    BUILD +envoy-container
    BUILD +keycloak-container
    BUILD +app-container
    BUILD +testing-container
    BUILD +operator-container
    BUILD +pipeline-job-container

npm-deps:
    COPY $PIPELINE_FOLDER/package.json $PIPELINE_FOLDER/package.json
    COPY $PIPELINE_FOLDER/package-lock.json $PIPELINE_FOLDER/package-lock.json
    RUN cd $PIPELINE_FOLDER && npm install
    SAVE ARTIFACT $PIPELINE_FOLDER/node_modules

npm-build:
    FROM +npm-deps
    COPY $PIPELINE_FOLDER $PIPELINE_FOLDER
    COPY +npm-deps/node_modules $PIPELINE_FOLDER/node_modules
    COPY --dir crates/ui-pages crates/ui-pages
    COPY --dir crates/daisy-rsx crates/daisy-rsx
    RUN cd $PIPELINE_FOLDER && npm run release
    SAVE ARTIFACT $PIPELINE_FOLDER/dist

prepare-cache:
    # Copy in all our crates
    COPY --dir crates crates
    COPY Cargo.lock Cargo.toml .
    RUN cargo chef prepare --recipe-path recipe.json --bin $AXUM_FOLDER
    SAVE ARTIFACT recipe.json
     

envoy-container:
    FROM $ENVOY_PROXY
    RUN mkdir -p /etc/envoy
    COPY .devcontainer/envoy/envoy.yaml /etc/envoy/envoy.yaml
    # The second development entry in our cluster list is the app
    RUN sed -i '0,/development/{s/development/app/}' /etc/envoy/envoy.yaml
    CMD ["/usr/local/bin/envoy","-c","/etc/envoy/envoy.yaml","--service-cluster","envoy","--service-node","envoy","--log-level","info"]
    SAVE IMAGE --push $ENVOY_IMAGE_NAME

keycloak-container:
    FROM $KEYCLOAK_BASE_IMAGE
    COPY .devcontainer/keycloak /opt/keycloak/data/import
    SAVE IMAGE --push $KEYCLOAK_IMAGE_NAME
     

pipeline-job-container:
    FROM scratch
    COPY +build/$PIPELINE_EXE_NAME pipeline-job
    ENTRYPOINT ["./pipeline-job"]
    SAVE IMAGE --push $PIPELINE_IMAGE_NAME

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
            && cargo build --release --target x86_64-unknown-linux-musl \
            && cargo test --no-run --release --target x86_64-unknown-linux-musl \
            && rm target/x86_64-unknown-linux-musl/release/deps/*.d \
            && mv target/x86_64-unknown-linux-musl/release/deps/single_user_test* single_user_test \
            && mv target/x86_64-unknown-linux-musl/release/deps/multi_user_test* multi_user_test
    END
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$APP_EXE_NAME
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$PIPELINE_EXE_NAME
    SAVE ARTIFACT multi_user_test
    SAVE ARTIFACT single_user_test

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
# docker run -it --rm -e APP_DATABASE_URL=$APP_DATABASE_URL -p 7403:7403 purtontech/trace-server:latest
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
    ENTRYPOINT ["./k8s-operator"]
    SAVE IMAGE --push $OPERATOR_IMAGE_NAME

# Package up the selenium tests into a container that we can
# run in the CI-CD pipeline
testing-container:
    FROM gcr.io/distroless/static
    COPY +build/multi_user_test multi_user_test
    COPY +build/single_user_test single_user_test
    COPY --dir .devcontainer/mocks ./mocks 
    COPY --dir .devcontainer/datasets ./datasets 
    CMD ./multi_user_test && ./single_user_test
    SAVE IMAGE --push $TESTING_IMAGE_NAME