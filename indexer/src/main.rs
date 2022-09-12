use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::{Context, Result};
use argh::FromArgs;
use serde::Deserialize;
use tokio::signal::unix;
use parking_lot::Mutex;

use indexer::archive_scanner::*;
use indexer::config::*;
use indexer::network_scanner::*;

#[global_allocator]
static GLOBAL: ton_indexer::alloc::Allocator = ton_indexer::alloc::allocator();

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
    match config.scan_type.clone() {
        ScanType::FromNetwork { node_config } => {
            let panicked = Arc::new(AtomicBool::default());
            let orig_hook = std::panic::take_hook();
            std::panic::set_hook({
                let panicked = panicked.clone();
                Box::new(move |panic_info| {
                    panicked.store(true, Ordering::Release);
                    orig_hook(panic_info);
                })
            });

            let global_config = ton_indexer::GlobalConfig::from_file(
                &app.global_config.context("Global config not found")?,
            )
            .context("Failed to open global config")?;

            init_logger(&config.logger_settings).context("Failed to init logger")?;

            log::info!("Initializing producer...");

            //let (shard_accounts_subscriber, current_key_block) = ShardAccountsSubscriber::new();
            let current_key_block :Arc<Mutex<Option<ton_block::Block>>> = Arc::new(Mutex::new(None));

            let engine = NetworkScanner::new(
                config.clone(),
                node_config,
                global_config,
            )
            .await
            .context("Failed to create engine")?;

            engine.start().await.context("Failed to start engine")?;
            {
                let last_key_block = engine.indexer().load_last_key_block().await?;
                let mut current_key_block = current_key_block.lock();
                if current_key_block.is_none() {
                    *current_key_block = Some(last_key_block.into_block());
                }
            }
            log::info!("Initialized engine");


            log::info!("Initialized producer");
            futures_util::future::pending().await
        }
        ScanType::FromArchives { list_path } => {
            let scanner = ArchivesScanner::new(config, list_path).await
                .context("Failed to create scanner")?;

            scanner.run().await.context("Failed to scan archives")
        }
    }
}

#[derive(Debug, PartialEq, FromArgs)]
#[argh(description = "A simple service to stream TON data into Kafka")]
struct App {
    /// path to config file ('config.yaml' by default)
    #[argh(option, short = 'c', default = "String::from(\"config.yaml\")")]
    config: String,

    /// path to global config file
    #[argh(option, short = 'g')]
    global_config: Option<String>,
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
