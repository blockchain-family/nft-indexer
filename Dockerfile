FROM europe-west1-docker.pkg.dev/blockchain-family/docker/rust-builder:v1.62 AS builder

WORKDIR /build

# Build App
COPY . .
RUN RUSTFLAGS=-g cargo build --release

FROM europe-west1-docker.pkg.dev/blockchain-family/docker/rust-runtime:v1.62
COPY --from=builder /build/target/release/model /app/application
COPY --from=builder /build/storage/migrations /app/migrations
COPY --from=builder /build/entrypoint.sh /app/entrypoint.sh
USER runuser
EXPOSE 9000
ENTRYPOINT ["/app/entrypoint.sh"]

