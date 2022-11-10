FROM europe-west1-docker.pkg.dev/blockchain-family/docker/rust-builder:v1.62 AS builder

WORKDIR /build

# Build App
COPY . .
RUN RUSTFLAGS=-g cargo build --release

FROM europe-west1-docker.pkg.dev/blockchain-family/docker/rust-runtime:v1.62
COPY --from=builder /build/target/release/model /app/application
COPY --from=builder /build/storage/migrations /app/migrations
COPY --from=builder /build/entrypoint.sh /app/entrypoint.sh
RUN base64 -d <<< "$KAFKA_KEYSTORE" > /app/broker.keystore.jks
RUN base64 -d <<< "$KAFKA_CA_PEM" > /app/ca.pem
USER runuser
EXPOSE 9000
ENTRYPOINT ["/app/entrypoint.sh"]
