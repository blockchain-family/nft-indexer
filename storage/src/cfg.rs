use serde::Deserialize;
use sqlx::{postgres::{PgPoolOptions, PgPool}, Error};

fn default_url() -> String {
    String::from("postgresql://localhost/nft_indexer")
}

fn default_max_connections() -> u32 {
    1
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    #[serde(default = "default_url")]
    pub url: String,

    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub async fn init(&self) -> Result<PgPool, Error>{
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(&self.url).await
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            url: default_url(),
            max_connections: default_max_connections(),
        }
    }
}

