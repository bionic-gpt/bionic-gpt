VERSION 0.7
FROM purtontech/rust-on-nails-devcontainer:1.1.15

ARG --global APP_EXE_NAME=axum-server
ARG --global EMBEDDINGS_EXE_NAME=embeddings-job
ARG --global DBMATE_VERSION=2.2.0

# Folders
ARG --global AXUM_FOLDER=crates/axum-server
ARG --global DB_FOLDER=crates/db
ARG --global PIPELINE_FOLDER=crates/asset-pipeline

# Base images
ARG --global ENVOY_PROXY=envoyproxy/envoy:v1.17-latest

# This file builds the following containers
ARG --global APP_IMAGE_NAME=purton-tech/bionicgpt:latest
ARG --global ENVOY_IMAGE_NAME=purton-tech/bionicgpt-envoy:latest
ARG --global MIGRATIONS_IMAGE_NAME=purton-tech/bionicgpt-db-migrations:latest
ARG --global EMBEDDINGS_IMAGE_NAME=purton-tech/bionicgpt-embeddings-job:latest

WORKDIR /build

USER vscode

dev:
    BUILD +pull-request
    # On github this check is performed directly by the action
    BUILD +check-selenium-failure

pull-request:
    BUILD +migration-container
    BUILD +app-container
    BUILD +envoy-container
    BUILD +embeddings-container

all:
    BUILD +migration-container
    BUILD +envoy-container
    BUILD +app-container
    BUILD +embeddings-container

npm-deps:
    COPY $PIPELINE_FOLDER/package.json $PIPELINE_FOLDER/package.json
    COPY $PIPELINE_FOLDER/package-lock.json $PIPELINE_FOLDER/package-lock.json
    RUN cd $PIPELINE_FOLDER && npm install
    SAVE ARTIFACT $PIPELINE_FOLDER/node_modules

npm-build:
    FROM +npm-deps
    COPY $PIPELINE_FOLDER $PIPELINE_FOLDER
    COPY +npm-deps/node_modules $PIPELINE_FOLDER/node_modules
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
    COPY .devcontainer/envoy.yaml /etc/envoy/envoy.yaml
    # The second development entry in our cluster list is the app
    RUN sed -i '0,/development/{s/development/app/}' /etc/envoy/envoy.yaml
    SAVE IMAGE --push $ENVOY_IMAGE_NAME
     

embeddings-container:
    FROM scratch
    COPY +build/$EMBEDDINGS_EXE_NAME embeddings-job
    ENTRYPOINT ["./embeddings-job"]
    SAVE IMAGE --push $EMBEDDINGS_IMAGE_NAME

build-cache:
    COPY +prepare-cache/recipe.json ./
    RUN cargo chef cook --release --target x86_64-unknown-linux-musl
    SAVE ARTIFACT target
    SAVE ARTIFACT $CARGO_HOME cargo_home
    SAVE IMAGE --cache-hint

build:
    # Copy in all our crates
    COPY --dir crates crates
    COPY --dir Cargo.lock Cargo.toml .
    COPY +build-cache/cargo_home $CARGO_HOME
    COPY +build-cache/target target
    COPY --dir +npm-build/dist $PIPELINE_FOLDER/
    # We need to run inside docker as we need postgres running for cornucopia
    ARG DATABASE_URL=postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable
    USER root
    WITH DOCKER \
        --pull ankane/pgvector
        RUN docker run -d --rm --network=host -e POSTGRES_PASSWORD=testpassword ankane/pgvector \
            && while ! pg_isready --host=localhost --port=5432 --username=postgres; do sleep 1; done ;\
                dbmate --migrations-dir $DB_FOLDER/migrations up \
            && cargo build --release --target x86_64-unknown-linux-musl
    END
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$APP_EXE_NAME
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$EMBEDDINGS_EXE_NAME

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

integration-test:
    FROM +build
    COPY .devcontainer/docker-compose.yml ./ 
    COPY .devcontainer/docker-compose.earthly.yml ./ 
    ARG DATABASE_URL=postgresql://postgres:testpassword@localhost:5432/test?sslmode=disable
    ARG APP_DATABASE_URL=postgresql://application:testpassword@db:5432/test
    # We expose selenium to localhost
    ARG WEB_DRIVER_URL='http://localhost:4444' 
    # The selenium container will connect to the envoy container
    ARG WEB_DRIVER_DESTINATION_HOST='http://envoy:7700' 
    # How do we connect to mailhog
    ARG MAILHOG_URL=http://localhost:8025/api/v2/messages?limit=1
    # Chnage they way we access internal API's
    ARG OPENAI_ENDPOINT=http://localhost:8080
    ARG UNSTRUCTURED_ENDPOINT=http://localhost:8000
    USER root
    WITH DOCKER \
        --compose docker-compose.yml \
        --compose docker-compose.earthly.yml \
        --service db \
        --service barricade \
        --service smtp \
        --service unstructured \
        # Do we need this? --service llm-api \
        # Record our selenium session
        --service selenium \
        --pull selenium/video:ffmpeg-4.3.1-20220208 \
        # Bring up the containers we have built
        --load $APP_IMAGE_NAME=+app-container \
        --load $ENVOY_IMAGE_NAME=+envoy-container

        # Force to command to always be succesful so the artifact is saved. 
        # https://github.com/earthly/earthly/issues/988
        RUN dbmate --wait-timeout 60s --migrations-dir $DB_FOLDER/migrations up \
            && docker run -d -p 7703:7703 --rm --network=build_default \
                -e APP_DATABASE_URL=$APP_DATABASE_URL \
                -e INVITE_DOMAIN=http://envoy:7700 \
                -e INVITE_FROM_EMAIL_ADDRESS=support@application.com \
                -e SMTP_HOST=smtp \
                -e SMTP_PORT=1025 \
                -e SMTP_USERNAME=thisisnotused \
                -e SMTP_PASSWORD=thisisnotused \
                -e SMTP_TLS_OFF='true' \
                --name app $APP_IMAGE_NAME \
            && docker run -d -p 7700:7700 --rm --network=build_default --name envoy $ENVOY_IMAGE_NAME \
            && cargo test --no-run --release --target x86_64-unknown-linux-musl \
            && docker run -d --name video --network=build_default -e DISPLAY_CONTAINER_NAME=build_selenium_1 -e FILE_NAME=chrome-video.mp4 -v /build/tmp:/videos selenium/video:ffmpeg-4.3.1-20220208 \
            && (cargo test --release --target x86_64-unknown-linux-musl -- --nocapture || echo fail > ./tmp/fail) \
            && docker ps \
            && docker stop video envoy app
    END
    # You need the tmp/* if you use just tmp earthly will overwrite the folder
    SAVE ARTIFACT tmp/* AS LOCAL ./tmp/earthly/

check-selenium-failure:
    FROM +integration-test
    # https://github.com/earthly/earthly/issues/988
    # If we failed in selenium a fail file will have been created
    # to get build to pass and see video, run +pull-request
    IF [ -f ./tmp/earthly/fail ]
        RUN echo "cargo test has failed." && exit 1
    END