FROM europe-west1-docker.pkg.dev/broxus-infrastructure/docker/rust-builder:stable AS builder

WORKDIR /build

# Build App
COPY . .
RUN RUSTFLAGS=-g cargo build --release

FROM europe-west1-docker.pkg.dev/blockchain-family/docker/rust-runtime:stable
COPY --from=builder /build/target/release/model /app/application
COPY --from=builder /build/indexer-repo/migrations /app/migrations
COPY --from=builder /build/indexer/src/abi/json /app/abi
COPY --from=builder /build/entrypoint.sh /app/entrypoint.sh
USER runuser
EXPOSE 3001
ENTRYPOINT ["/app/entrypoint.sh"]

