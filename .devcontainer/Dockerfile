FROM mcr.microsoft.com/devcontainers/rust:bookworm

RUN apt-get update && apt-get install -y --no-install-recommends \
    pipx \
    vim

USER vscode

RUN pipx ensurepath && pipx install pre-commit
