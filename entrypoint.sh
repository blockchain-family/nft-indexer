#!/bin/bash

echo "$KAFKA_KEYSTORE" | base64 -d > /app/broker.keystore.jks
echo "$KAFKA_CA_PEM" | base64 -d > /app/ca.pem

sqlx migrate run && /app/application $1
