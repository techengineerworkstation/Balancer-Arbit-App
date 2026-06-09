# Build stage
FROM rust:1.82-slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY web-app/ web-app/

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash botuser
WORKDIR /app

COPY --from=builder /app/target/release/balancer-arb-server .
COPY --from=builder /app/web-app/dist/ web-app/dist/
COPY --from=builder /app/config.json ./config.json

RUN chown -R botuser:botuser /app
USER botuser

EXPOSE 8080

ENV RUST_LOG=info
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080

HEALTHCHECK --interval=30s --timeout=10s --retries=3 \
    CMD curl -f http://localhost:8080/api/status || exit 1

CMD ["./balancer-arb-server"]