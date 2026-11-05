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

ENV HOST=0.0.0.0
ENV PORT=2222

ENTRYPOINT ["/sbin/tini", "--", "/app/humboldti"]
