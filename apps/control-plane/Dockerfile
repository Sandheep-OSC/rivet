# Build stage
FROM rust:1.82-slim AS builder

WORKDIR /app

# Copy workspace files
COPY . .

# Build the control-plane
RUN cargo build --release --package control-plane

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/control-plane .

EXPOSE 8080

CMD ["./control-plane"]