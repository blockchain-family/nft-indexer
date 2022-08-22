use std::collections::HashMap;
use anyhow::{Context, Result};
use argh::FromArgs;
use serde::Deserialize;
use tokio::signal::unix;
use futures_util::StreamExt;

use ton_block::Transaction;
use kafka_indexer::config::AppConfig;
use model::subscriber::IndexerSubscriber;
use transaction_consumer::{TransactionConsumer, ConsumerOptions, StreamFrom};


#[tokio::main]
async fn main() -> Result<()> {
    let any_signal = any_signal([
        unix::SignalKind::interrupt(),
        unix::SignalKind::terminate(),
        unix::SignalKind::quit(),
        unix::SignalKind::from_raw(6),  // SIGABRT/SIGIOT
        unix::SignalKind::from_raw(20), // SIGTSTP
    ]);

    let run = run(argh::from_env());

    tokio::select! {
        result = run => result,
        signal = any_signal => {
            if let Ok(signal) = signal {
                log::warn!("Received signal ({:?}). Flushing state...", signal);
            }
            // NOTE: engine future is safely dropped here so rocksdb method
            // `rocksdb_close` is called in DB object destructor
            Ok(())
        }
    }
}

async fn run(app: App) -> Result<()> {
    let config: AppConfig = read_config(app.config)?;
    countme::enable(true);

    tokio::spawn(memory_profiler());

    let consumer_options = ConsumerOptions {
        skip_0_partition: false,
        kafka_options: config.kafka.options
                        .iter()
                        .map(|(x, y)| (x.as_str(), y.as_str()))
                        .collect::<HashMap<_, _>>(),
    };
    let transaction_producer = TransactionConsumer::new(
        &config.kafka.group_id,
        &config.kafka.topic,
        vec![&config.kafka.endpoint],
        None,
        consumer_options,
    ).await.expect("fail get transaction producer");

    log::debug!("await get stream_transaction");
    let mut stream_transactions = transaction_producer
        .clone()
        .stream_transactions(StreamFrom::Beginning)
        .await
        .unwrap();

    let subscriber = IndexerSubscriber::new2(config.model).await?;

    while let Some(produced_transaction) = stream_transactions.next().await {
        let transaction: Transaction = produced_transaction.transaction.clone();
        subscriber.handle_transaction(transaction).await?;
    }

    Ok(())
}

#[derive(Debug, PartialEq, FromArgs)]
#[argh(description = "A simple service to stream TON data into Kafka")]
struct App {
    /// path to config file ('config.yaml' by default)
    #[argh(option, short = 'c', default = "String::from(\"config.yaml\")")]
    config: String,
}

fn any_signal<I>(signals: I) -> tokio::sync::oneshot::Receiver<unix::SignalKind>
where
    I: IntoIterator<Item = unix::SignalKind>,
{
    let (tx, rx) = tokio::sync::oneshot::channel();

    let any_signal = futures_util::future::select_all(signals.into_iter().map(|signal| {
        Box::pin(async move {
            unix::signal(signal)
                .expect("Failed subscribing on unix signals")
                .recv()
                .await;
            signal
        })
    }));

    tokio::spawn(async move {
        let signal = any_signal.await.0;
        tx.send(signal).ok();
    });

    rx
}

fn read_config<P, T>(path: P) -> Result<T>
where
    P: AsRef<std::path::Path>,
    for<'de> T: Deserialize<'de>,
{
    let data = std::fs::read_to_string(path).context("Failed to read config")?;
    let re = regex::Regex::new(r"\$\{([a-zA-Z_][0-9a-zA-Z_]*)\}").unwrap();
    let result = re.replace_all(&data, |caps: &regex::Captures| {
        match std::env::var(&caps[1]) {
            Ok(value) => value,
            Err(_) => {
                log::warn!("Environment variable {} was not set", &caps[1]);
                String::default()
            }
        }
    });

    config::Config::builder()
        .add_source(config::File::from_str(
            result.as_ref(),
            config::FileFormat::Yaml,
        ))
        .build()
        .context("Failed to load config")?
        .try_deserialize()
        .context("Failed to parse config")
}

fn init_logger(config: &serde_yaml::Value) -> Result<()> {
    let config = serde_yaml::from_value(config.clone())?;
    log4rs::config::init_raw_config(config)?;
    Ok(())
}

async fn memory_profiler() {
    use ton_indexer::alloc;

    let signal = unix::SignalKind::user_defined1();
    let mut stream = unix::signal(signal).expect("failed to create signal stream");
    let path = std::env::var("MEMORY_PROFILER_PATH").unwrap_or_else(|_| "memory.prof".to_string());
    let mut is_active = false;
    while stream.recv().await.is_some() {
        log::info!("Memory profiler signal received");
        if !is_active {
            log::info!("Activating memory profiler");
            if let Err(e) = alloc::activate_prof() {
                log::error!("Failed to activate memory profiler: {}", e);
            }
        } else {
            let invocation_time = chrono::Local::now();
            let path = format!("{}_{}", path, invocation_time.format("%Y-%m-%d_%H-%M-%S"));
            if let Err(e) = alloc::dump_prof(&path) {
                log::error!("Failed to dump prof: {:?}", e);
            }
            if let Err(e) = alloc::deactivate_prof() {
                log::error!("Failed to deactivate memory profiler: {}", e);
            }
        }

        is_active = !is_active;
    }
}
