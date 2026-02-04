ARG PORT=8080
ARG NODE_VERSION=24

#########################################################
###                  NODE DEPENDENCIES                ###
#########################################################
FROM node:${NODE_VERSION} AS node_dependencies
WORKDIR /app
COPY . .
RUN make assets

#########################################################
###                      CHEF SETUP                   ###
#########################################################
FROM rust:1 AS chef
RUN cargo install cargo-chef
RUN rustup target add wasm32-unknown-unknown
WORKDIR /app

#########################################################
###                      CHEF CACHE                   ###
#########################################################
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

#########################################################
###                        BUNDLE                     ###
#########################################################
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo chef cook --release --no-build --recipe-path recipe.json

# Install dx and dx dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl ca-certificates pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli@$(cargo tree -i dioxus --depth 0 | grep -oE '[0-9]+\.[0-9]+\.[0-9]+') --root /root/.cargo --force --disable-telemetry
ENV PATH="/root/.cargo/bin:$PATH"


COPY . .
COPY --from=node_dependencies /app/packages/frontend/assets/ packages/frontend/assets/
RUN rm /app/packages/frontend/tailwind.css # Avoid dioxus recreating tailwind.css on build, since it is generated in the node_dependencies stage
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    make bundle-no-deps PLATFORM=web

#########################################################
###                          APP                      ###
#########################################################
FROM debian:trixie-slim
ENV IP=0.0.0.0
ENV PORT=${PORT}
EXPOSE ${PORT}

WORKDIR /app

RUN useradd -m -u 10001 roommates

COPY --from=builder --chown=roommates:roommates /app/dist/web .
RUN chown -R roommates:roommates /app
USER roommates

CMD ["./frontend"]
