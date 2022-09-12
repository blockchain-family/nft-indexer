## ton-kafka-producer

### Runtime requirements

- CPU: 4 cores, 2 GHz
- RAM: 8 GB
- Storage: 100 GB fast SSD
- Network: 100 MBit/s

### How to run

1. Build all binaries and prepare services
   ```bash
   ./scripts/setup.sh
   ```
2. Edit `/etc/nft-indexer/config.yaml`
3. Enable and start the service:
   ```bash
   systemctl enable nft-indexer
   systemctl start nft-indexer
   ```

### Config example
```yaml
---
rpc_config:
  # States RPC endpoint
  address: "0.0.0.0:3087"

scan_type:
  kind: FromNetwork
  node_config:
    # Root directory for node DB. Default: "./db"
    # db_path: "/var/db/ton-kafka-producer"

    # UDP port, used for ADNL node. Default: 30303
    adnl_port: 30303

    # Path to temporary ADNL keys.
    # NOTE: Will be generated if it was not there.
    # Default: "./adnl-keys.json"
    # temp_keys_path: "/etc/nft-indexer/adnl-keys.json"

    # Archives map queue. Default: 16
    parallel_archive_downloads: 6

    # # Specific block from which to run the indexer
    start_from: 4296289

    # # Allowed DB size in bytes. Default: one third of all machine RAM
    max_db_memory_usage: 10000000000

model:
  database:
    url: postgresql://localhost/nft_indexer
    max_connections: 1
  auctions:
    contracts:
      - ааааа


# log4rs settings.
# See https://docs.rs/log4rs/1.0.0/log4rs/ for more details
logger_settings:
  appenders:
    stdout:
      kind: console
      encoder:
        pattern: "{h({l})} {M} = {m} {n}"
  root:
    level: error
    appenders:
      - stdout
  loggers:
    ton_indexer:
      level: info
      appenders:
        - stdout
      additive: true
    indexer:
      level: info
      appenders:
        - stdout
      additive: true
```

### States RPC

Endpoint: `http://0.0.0.0:8081` (can be configured by `rpc_config.address`)

- POST `/account`
  ```typescript
  type Request = {
    // Address of the contract in format `(?:-1|0):[0-9a-fA-F]{64}`
    address: string,
  };
  
  type Response = {
    // BOC encoded `ton_block::AccountStuff`
    account: string,
    timings: {
      // Shard state lt
      genLt: string,
      // Shard state utime
      genUtime: number,
    },
    lastTransactionId: {
      // Account last transaction lt
      lt: string,
      // Account last transaction hash in format `[0-9a-f]{64}`
      hash: string,
    }
  } | null;
  ```
