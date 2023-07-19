FROM europe-west1-docker.pkg.dev/blockchain-family/docker/rust-builder:v1.68.0 AS builder

WORKDIR /build

# Build App
COPY . .
RUN RUSTFLAGS=-g cargo build --release

FROM europe-west1-docker.pkg.dev/blockchain-family/docker/rust-runtime:v1.68.0
COPY --from=builder /build/target/release/model /app/application
COPY --from=builder /build/indexer-repo/migrations /app/migrations
COPY --from=builder /build/indexer/src/abi/json /app/abi
COPY --from=builder /build/entrypoint.sh /app/entrypoint.sh
USER runuser
EXPOSE 3001
ENTRYPOINT ["/app/entrypoint.sh"]

