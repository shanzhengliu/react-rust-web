# node build start
FROM oven/bun:canary-alpine AS NodeBuild
WORKDIR /app
COPY ./frontend-react/ /app
RUN bun upgrade
RUN bun install
RUN bun run build
# node build end


FROM rust:1.77.2-alpine AS build
ARG APP_NAME
WORKDIR /app
RUN mkdir /app/static
# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git
COPY --from=NodeBuild /app/dist /app/static
RUN --mount=type=bind,source=backend-rust/src,target=src \
    --mount=type=bind,source=backend-rust/Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=backend-rust/Cargo.lock,target=Cargo.lock \
cargo build --locked --release && \
cp ./target/release/backend-rust /bin/server

FROM alpine:3.18 AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
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

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
CMD ["/bin/server"]
