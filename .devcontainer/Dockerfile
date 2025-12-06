FROM mcr.microsoft.com/devcontainers/rust:trixie
ARG NODE_VERSION=20

RUN apt-get update && apt-get install -y --no-install-recommends \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    pipx \
    vim

USER vscode

#Dioxus
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli@0.7.2
RUN rustup target add wasm32-unknown-unknown
RUN cargo install sea-orm-cli@^2.0.0-rc



# Node
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
ENV NVM_DIR=/home/vscode/.nvm
RUN bash -c "source $NVM_DIR/nvm.sh && nvm install $NODE_VERSION"

RUN pipx ensurepath && pipx install pre-commit
