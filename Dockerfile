ARG APP_NAME=backend_lab_3

# Builder stage
ARG RUST_VERSION=1.90.0
FROM rust:${RUST_VERSION}-slim AS builder

# Leverage mounts to speed up the build process
ARG APP_NAME
WORKDIR /app
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    set -e; \
    cargo build --locked --release && \
    cp ./target/release/${APP_NAME} /bin/app

# Runner stage
FROM gcr.io/distroless/cc:nonroot AS runner

COPY --from=builder /bin/app /bin/

ARG PORT=8080
ENV PORT=${PORT}
EXPOSE ${PORT}

CMD ["/bin/app"]
