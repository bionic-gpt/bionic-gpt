FROM purtontech/rust-on-nails-devcontainer:1.1.12 AS builder

# set the workdir and copy the source into it
WORKDIR /home/vscode/

# Get the model early so we don't have to break the cache if we do stuff further down.
# It's 5GB
RUN wget --no-verbose https://huggingface.co/rustformers/mpt-7b-ggml/resolve/main/mpt-7b-chat-q5_1-ggjt.bin

# OpenSSL
RUN sudo apt-get update \
    && sudo apt-get install -y \
    openssl \
    pkg-config \
    libssl-dev

# Clone the repo
RUN git clone --recurse-submodules --depth 1 https://github.com/rustformers/llm.git
# do a release build
WORKDIR /home/vscode/llm
RUN cargo build --release --bin llm
RUN strip target/release/llm

# Use a slim image
FROM rust:slim-bookworm
# copy the binary into the final image
COPY --from=builder /home/vscode/llm/target/release/llm .
COPY --from=builder /home/vscode/mpt-7b-chat-q5_1-ggjt.bin .

# set the binary as entrypoint
ENTRYPOINT ["/llm",  "mpt", "chat", "--model-path", "/mpt-7b-chat-q5_1-ggjt.bin"]