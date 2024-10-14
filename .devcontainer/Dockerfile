FROM purtontech/rust-on-nails-devcontainer:1.3.3 AS development

COPY ps1.bash .
RUN cat ps1.bash >> ~/.bashrc && sudo rm ps1.bash

COPY bash_aliases .
RUN cat bash_aliases >> ~/.bash_aliases && sudo rm bash_aliases

# Enable our git hooks and set the permisisons on docker sock.
RUN echo 'git config core.hooksPath /workspace/.devcontainer/.githooks' >> ~/.bashrc

RUN sudo apt-get update && sudo apt-get install -y python3 python3-pip python3-venv && \
    python3 -m venv ~/venv && \
    . ~/venv/bin/activate && \
    python -m pip install -U pip aider-chat && \
    echo 'export PATH="$HOME/venv/bin:$PATH"' >> ~/.bashrc && \
    . ~/.bashrc