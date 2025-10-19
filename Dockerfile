# Builder stage
ARG RUST_VERSION=1.90.0
ARG APP_NAME=backend_lab_2
FROM rust:${RUST_VERSION}-slim-bullseye AS builder
ARG APP_NAME
WORKDIR /app

# Leverage mounts to speed up the build process
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    set -e; \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

# Runner stage
FROM debian:bullseye-slim AS runner

# Create a non-privileged user the app will run under
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser
COPY --from=builder /bin/server /bin/

ARG PORT=8080
ENV PORT=${PORT}
EXPOSE ${PORT}

CMD ["/bin/server"]
