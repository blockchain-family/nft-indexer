#!/bin/bash

echo "$KAFKA_KEYSTORE" | base64 -d > /app/broker.keystore.jks
echo "$KAFKA_CA_PEM" | base64 -d > /app/ca.pem

echo $SSL_CA | base64 -d > /tmp/ca.pem
echo $SSL_KEY | base64 -d > /tmp/service.key
echo $SSL_CERTIFICATE | base64 -d > /tmp/service.crt

sqlx migrate run && /app/application $1
