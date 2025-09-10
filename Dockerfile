# Build stage
FROM rust:alpine AS builder
WORKDIR /app
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo build --release --target x86_64-unknown-linux-musl

# Final stage
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rsapp /rsapp
CMD ["/rsapp"]

# FROM scratch
# COPY ./target/x86_64-unknown-linux-musl/release/rsapp /rsapp
# CMD ["/rsapp"]