FROM hub.broxus.com/broxus/infrastructure/docker/rust-builder:stable AS builder

WORKDIR /build

COPY . .
RUN RUSTFLAGS=-g cargo build --release


FROM hub.broxus.com/broxus/infrastructure/docker/rust-runtime:stable AS runtime
COPY --from=builder /build/target/release/nft-indexer /app/application
COPY --from=builder /build/indexer-repo/migrations /app/migrations
COPY --from=builder /build/indexer/src/abi/json /app/abi
COPY --from=builder /build/entrypoint.sh /app/entrypoint.sh

RUN groupadd -g 10001 app \
    && useradd -u 10001 -g 10001 -d /app -s /usr/sbin/nologin app \
    && chown -R 10001:10001 /app

USER 10001:10001
EXPOSE 3001
ENTRYPOINT ["/app/entrypoint.sh"]