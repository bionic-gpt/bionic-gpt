FROM purtontech/rust-on-nails-devcontainer:1.1.17 AS development

ENV NVIDIA_DRIVER_CAPABILITIES compute,utility,graphics

COPY ps1.bash .
RUN cat ps1.bash >> ~/.bashrc && sudo rm ps1.bash

COPY .bash_aliases /home/vscode/.bash_aliases

# Enable our git hooks and set the permisisons on docker sock.
RUN echo 'git config core.hooksPath /workspace/.devcontainer/.githooks' >> ~/.bashrc

# Zola
# Minimum supported cloudflare version
ARG ZOLA_VERSION=0.17.2
RUN sudo curl -OL https://github.com/getzola/zola/releases/download/v$ZOLA_VERSION/zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    && sudo tar -xvf zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    && sudo mv zola /usr/bin/zola \
    && sudo chmod +x /usr/bin/zola

# all the volumes configured in the docker-compose.yml
ARG USERNAME=vscode
RUN sudo mkdir -p /workspace/target && sudo chown $USERNAME:$USERNAME /workspace/target

RUN sudo curl -OL https://dl.min.io/client/mc/release/linux-amd64/mc \
    && sudo chmod +x mc \
    && sudo mv ./mc /usr/bin

# We need this for candle-llama
RUN sudo apt-get install -y pkg-config

RUN curl -fsSL https://raw.githubusercontent.com/metalbear-co/mirrord/main/scripts/install.sh | bash