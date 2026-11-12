# ---- Builder Stage ----
FROM rust:alpine AS builder

WORKDIR /build

# Cache dependency compilation in its own layer
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
  && cargo build --release \
  && rm -rf src target/release/humboldti

# Real source
COPY src/ src/
RUN touch src/main.rs && cargo build --release

# ---- Final Stage ----
FROM alpine
RUN apk add --no-cache tini

WORKDIR /app

COPY --from=builder /build/target/release/humboldti humboldti

EXPOSE 2222/tcp
EXPOSE 2222/udp

ENTRYPOINT ["/sbin/tini", "--", "/app/humboldti"]
