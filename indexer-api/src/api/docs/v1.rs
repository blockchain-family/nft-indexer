#![allow(clippy::needless_update)]

use actix_web::{get, web};
use opg::*;

use crate::api::metadata::RefreshMetadataParams;

pub fn swagger(api_url: &str) -> Opg {
    describe_api! {
        info: {
            title: "Indexer builtin API",
            version: "0.0.1",
            description: "Provides manual management"
        },
        tags: {
            metadata
        },
        servers: {
            api_url
        },
        paths: {
            ("metadata" / "refresh"): {
                POST: {
                    tags: { metadata },
                    summary: "Manual meta refresh of nfts and collections",
                    body: RefreshMetadataParams,
                }
            },
        }
    }
}

#[get("/swagger.yaml")]
pub async fn swagger_yaml(api_url: web::Data<String>) -> String {
    let api = swagger(&api_url);
    serde_yaml::to_string(&api).unwrap()
}

#[get("/swagger.json")]
pub async fn swagger_json(api_url: web::Data<String>) -> String {
    let api = swagger(&api_url);
    serde_json::to_string(&api).unwrap()
}
