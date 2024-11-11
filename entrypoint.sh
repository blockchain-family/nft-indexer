#!/bin/bash

echo "$KAFKA_KEYSTORE" | base64 -d > /app/broker.keystore.jks
echo "$KAFKA_CA_PEM" | base64 -d > /app/ca.pem

env | grep 'SSL_CA' | cut -d'=' -f2- | base64 -d > /tmp/ca.pem
env | grep 'SSL_KEY' | cut -d'=' -f2- | base64 -d > /tmp/service.key
env | grep 'SSL_CERTIFICATE' | cut -d'=' -f2- | base64 -d > /tmp/service.crt

sqlx migrate run && /app/application $1
