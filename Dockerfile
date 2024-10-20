FROM europe-west1-docker.pkg.dev/broxus-infrastructure/docker/rust-builder:stable AS builder

WORKDIR /build

# Build dependencies only, when source code changes,
# this build can be cached, we don't need to compile dependency again.
RUN mkdir src && touch src/lib.rs
COPY data-reader data-reader
COPY indexer indexer
COPY indexer-api indexer-api
COPY indexer-jobs indexer-jobs
COPY indexer-repo indexer-repo
COPY Cargo.toml Cargo.lock ./
RUN RUSTFLAGS=-g cargo build --release

# Build App
COPY . .
RUN RUSTFLAGS=-g cargo build --release

FROM europe-west1-docker.pkg.dev/broxus-infrastructure/docker/rust-runtime:stable
COPY --from=builder /build/target/release/nft-indexer /app/application
COPY --from=builder /build/indexer-repo/migrations /app/migrations
COPY --from=builder /build/indexer/src/abi/json /app/abi
COPY --from=builder /build/entrypoint.sh /app/entrypoint.sh
USER runuser
EXPOSE 3001
ENTRYPOINT ["/app/entrypoint.sh"]
