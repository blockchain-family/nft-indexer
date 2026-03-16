#!/bin/bash
set -e

write_secret_file() {
  local value="$1"
  local path="$2"

  [ -n "$value" ] || return 0

  case "$value" in
    *"-----BEGIN "*)
      printf '%s\n' "$value" > "$path"
      ;;
    *)
      printf '%s' "$value" | base64 -d > "$path"
      ;;
  esac
}

write_secret_file "${SSL_CA:-}" /tmp/ca.pem
write_secret_file "${SSL_KEY:-}" /tmp/service.key
write_secret_file "${SSL_CERTIFICATE:-}" /tmp/service.crt

sqlx migrate run
exec /app/application "$@"
